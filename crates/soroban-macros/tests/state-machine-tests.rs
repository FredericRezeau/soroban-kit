/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

/// Integration tests for the soroban-macros state-machine module (`fsm`).
#[cfg(feature = "state-machine")]
mod tests {

    extern crate soroban_tools;
    extern crate std;

    use core::panic::AssertUnwindSafe;
    use soroban_sdk::{
        contract, contractimpl, contracttype, testutils::Address as _, Address, Env,
    };

    use soroban_macros::state_machine;
    use soroban_tools::{
        fsm,
        fsm::{StateMachine, TransitionHandler},
    };

    use std::panic::catch_unwind;

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Game {
        WorldOfWarcraft,
        LeagueOfLegends,
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum State {
        Opened,
        Ready,
        Playing(Game),
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Room {
        Public,
        Private(Address),
    }

    pub struct GamingLobby;

    impl TransitionHandler<Room, State> for GamingLobby {
        // Called immediately before state validation.
        // Used to implement guard conditions for the transition.
        fn on_guard(&self, env: &Env, state_machine: &StateMachine<Room, State>) {
            // E.g., Use this handler to implement time based transitions.

            let state = state_machine.get_state(&env).unwrap();
            let room = state_machine.get_region();
            match room {
                Room::Public => {
                    assert_eq!(state, State::Opened);
                }
                _ => match state {
                    State::Ready
                    | State::Playing(Game::WorldOfWarcraft)
                    | State::Playing(Game::LeagueOfLegends) => {}
                    _ => panic!("Invalid state."),
                },
            }
        }

        // Called immediately after state validation iff validation succeeded.
        // Used to implement the effect (immediate action) from transitioning.
        // This code block has precedence over the attributed function body.
        fn on_effect(&self, _env: &Env, _state_machine: &StateMachine<Room, State>) {}
    }

    impl GamingLobby {
        // This function panics when gaming lobby state is not State:Opened.
        #[state_machine(state = "State:Opened", region = "Room:Public")]
        fn login(&self, env: &Env, account: &Address) {
            self.set_state(&env, State::Ready, &account);
        }

        // This function panics when gaming lobby state is not State:Ready.
        #[state_machine(
            state = "State:Ready",
            region = "Room:Private:account"
        )]
        fn play(&self, env: &Env, account: &Address, item: &Game) {
            self.set_state(&env, State::Playing(item.clone()), &account);
        }

        // This function panics when gaming lobby
        // state is not State:Playing(Game), the tuple runtime value must match.
        #[state_machine(
            state = "State:Playing:item",
            region = "Room:Private:account"
        )]
        fn rage_quit(&self, env: &Env, account: &Address, item: &Game) {}

        // This function panics when gaming lobby
        // state is not State:Playing(Game), the tuple runtime value must match.
        #[state_machine(
            state = "State:Playing:item",
            region = "Room:Private:account"
        )]
        fn quit(&self, env: &Env, account: &Address, item: &Game) {}

        fn set_state(&self, env: &Env, state: State, account: &Address) {
            let region = Room::Private(account.clone());
            let state_machine =
                StateMachine::<Room, State>::new(&region, fsm::StorageType::Instance);
            state_machine.set_state(&env, state);
        }

        fn open(&self, env: &Env) {
            let state_machine =
                StateMachine::<Room, State>::new(&Room::Public, fsm::StorageType::Instance);
            state_machine.set_state(&env, State::Opened);
        }
    }

    #[contract]
    pub struct TestContract;

    #[contractimpl]
    impl TestContract {
        pub fn test_state_machine(env: Env) {
            // soroban-kit FSM allows modeling complex concurrency behaviors
            // with regions and extended state variables.

            let player1 = Address::generate(&env);
            let player2 = Address::generate(&env);
            let gaming_lobby = GamingLobby;

            // Set the initial state to Opened.
            gaming_lobby.open(&env);

            // We use regions to create a composite state (see #[state_machine] attribute).
            // so players login state is managed concurrently.
            gaming_lobby.login(&env, &player1);
            gaming_lobby.login(&env, &player2);

            // We use extended state variables to model any number of games.
            gaming_lobby.play(&env, &player1, &Game::WorldOfWarcraft);
            gaming_lobby.play(&env, &player2, &Game::LeagueOfLegends);

            // e.g., Player1 tries to rage_quit LeagueOfLegends (not currently playing).
            let result = catch_unwind(AssertUnwindSafe(|| {
                gaming_lobby.rage_quit(&env, &player1, &Game::LeagueOfLegends);
            }));
            assert!(
                result.is_err(),
                "The operation should panic. Player1 is playing WorldOfWarcraft"
            );

            gaming_lobby.quit(&env, &player1, &Game::WorldOfWarcraft);
            gaming_lobby.quit(&env, &player2, &Game::LeagueOfLegends);
        }
    }

    #[test]
    fn test_macros_state_machine() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_state_machine();
    }
}
