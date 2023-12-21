/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate std;
use core::panic::AssertUnwindSafe;
use soroban_sdk::{
    bytes, contract, contractimpl, contracttype, symbol_short,
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient as TokenAdminClient},
    vec, Address, Bytes, BytesN, Env, Symbol, Vec,
};

use std::{panic::catch_unwind, println};

use soroban_kit::{
    commit, fsm,
    fsm::StateMachine,
    key_constraint,
    oracle::{Envelope, Events},
    oracle_subscriber, reveal, soroban_tools, state_machine, storage, when_closed, when_opened,
    CircuitBreaker, TransitionHandler,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

// Use `key_constraint` to apply a constraint to the Key.
#[contracttype]
#[key_constraint(HelloKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    Newcomer,
}

// Use `storage` to implement the desired storage for your
// custom contract type. We also apply the HelloKeyConstraint.
#[contracttype]
#[storage(Instance, HelloKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    pub newcomer: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    Americano,
    Cappuccino,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Refilled,
    Selection,
    Distribution(Item),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Region {
    All,
    Buyer(Address),
}

#[derive(TransitionHandler)]
pub struct CoffeeMachine;

impl CoffeeMachine {
    // Called immediately before state validation.
    // Used to implement guard conditions for the transition.
    fn on_guard(&self, env: &Env, state_machine: &StateMachine<Region, State>) {
        // E.g., Use this handler to implement time based transitions.
        println!(
            "TransitionHandler::on_guard. State = {:?} Region = {:?} Storage = {:?} Timestamp = {}",
            state_machine.get_state(&env).unwrap(),
            state_machine.get_region(),
            state_machine.get_storage_type(),
            env.ledger().timestamp()
        );
    }

    // Called immediately after state validation iff validation succeeded.
    // Used to implement the effect (immediate action) from transitioning.

    // fn on_effect(&self, _env: &Env, _state_machine: &StateMachine<Region, State>) {
    //     // This code block has precedence over the `state_machine`
    //     // attributed function body.
    // }

    // This function panics when vending machine
    // state is not State:Refilled
    #[state_machine(
        state = "State:Refilled",
        region = "Region:All",
        transition = true,
        storage = "temporary"
    )]
    fn insert_coin(&self, env: &Env, account: &Address) {
        self.set_state(&env, State::Selection, &account);
    }

    // This function panics when vending machine
    // state is not State:Selection.
    #[state_machine(
        state = "State:Selection",
        region = "Region:Buyer:account",
        transition = true,
        storage = "temporary"
    )]
    fn select(&self, env: &Env, account: &Address, item: &Item) {
        self.set_state(&env, State::Distribution(item.clone()), &account);
    }

    // This function panics when vending machine
    // state is not State:Distribution(Item), the tuple runtime value must match.
    #[state_machine(
        state = "State:Distribution:item",
        region = "Region:Buyer:account",
        transition = true,
        storage = "temporary"
    )]
    fn distribute(&self, env: &Env, account: &Address, item: &Item) {}

    fn set_state(&self, env: &Env, state: State, account: &Address) {
        let region = Region::Buyer(account.clone());
        let state_machine =
            StateMachine::<Region, State>::new(&region, fsm::StorageType::Temporary);
        state_machine.set_state(&env, &state);
    }

    fn refill(&self, env: &Env) {
        let state_machine =
            StateMachine::<Region, State>::new(&Region::All, fsm::StorageType::Temporary);
        state_machine.set_state(&env, &State::Refilled);
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Alice,
    Bob,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    Start,
    Committing(Player),
    Revealing(Player),
    Completed(Player),
    End,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Domain {
    Players(Player),
    Game,
}

#[derive(TransitionHandler)]
pub struct RockPaperScissors;

impl RockPaperScissors {
    fn on_effect(&self, env: &Env, state_machine: &StateMachine<Domain, Phase>) {
        let domain = state_machine.get_region();
        let phase = state_machine.get_state(&env).unwrap();
        match domain {
            Domain::Players(Player::Alice) => {
                match phase {
                    Phase::Committing(Player::Alice) => {
                        // Alice played, transition to revealing phase.
                        state_machine.set_state(&env, &Phase::Revealing(Player::Alice));
                    }
                    Phase::Revealing(Player::Alice) => {
                        // Alice revealed, transition to completed phase.
                        state_machine.set_state(&env, &Phase::Completed(Player::Alice));
                        // If Bob also played, set game to End phase.
                        end_game_if_completed(&env, Player::Alice, Player::Bob);
                    }
                    _ => {}
                }
            }
            Domain::Players(Player::Bob) => {
                match phase {
                    Phase::Committing(Player::Bob) => {
                        // Bob played, transition to revealing phase.
                        state_machine.set_state(&env, &Phase::Revealing(Player::Bob));
                    }
                    Phase::Revealing(Player::Bob) => {
                        // Bob revealed, transition to completed phase.
                        state_machine.set_state(&env, &Phase::Completed(Player::Bob));
                        // If Alice also played, set game to End phase.
                        end_game_if_completed(&env, Player::Bob, Player::Alice);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        // Game completion helper.
        fn end_game_if_completed(env: &Env, for_player: Player, check_player: Player) {
            let state_machine =
                StateMachine::<Domain, Phase>::new(&Domain::Game, fsm::StorageType::Instance);
            let next_phase = match state_machine.get_state(env).unwrap_or(Phase::Start) {
                Phase::Completed(player) if player == check_player => Phase::End,
                _ => Phase::Completed(for_player),
            };
            state_machine.set_state(env, &next_phase);
        }
    }

    fn on_guard(&self, _env: &Env, _state_machine: &StateMachine<Domain, Phase>) {
        // Here you could implement a time-based guard to restrict the amount of time
        // allowed for players and so on...
    }

    #[commit]
    #[state_machine(state = "Phase:Committing:player", region = "Domain:Players:player")]
    fn play(&self, env: &Env, player: &Player, hash: &BytesN<32>) {}

    #[reveal]
    #[state_machine(state = "Phase:Revealing:player", region = "Domain:Players:player")]
    fn reveal(&self, env: &Env, player: &Player, data: &Bytes) {}

    #[state_machine(state = "Phase:End", region = "Domain:Game")]
    fn solve(&self, env: &Env) {}

    fn reset_player(&self, env: &Env, player: Player) {
        let domain = Domain::Players(player.clone());
        let phase = Phase::Committing(player.clone());
        let state_machine = StateMachine::<Domain, Phase>::new(&domain, fsm::StorageType::Instance);
        state_machine.set_state(&env, &phase);
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CircuitRegion {
    Users(Address),
}

#[derive(CircuitBreaker)]
pub struct Circuit;

impl Circuit {
    #[when_opened]
    fn can_call_when_opened(&self, env: &Env) {}

    #[when_closed]
    fn can_call_when_closed(&self, env: &Env) {}

    #[when_opened(trigger = true)]
    fn set_closed(&self, env: &Env) {}

    #[when_closed(trigger = true)]
    fn set_opened(&self, env: &Env) {}

    #[when_opened(region = "CircuitRegion:Users:addr")]
    fn can_call_when_opened_for_addr(&self, env: &Env, addr: &Address) {}

    #[when_closed(region = "CircuitRegion:Users:addr")]
    fn can_call_when_closed_for_addr(&self, env: &Env, addr: &Address) {}

    #[when_opened(trigger = true, region = "CircuitRegion:Users:addr")]
    fn set_closed_for_addr(&self, env: &Env, addr: &Address) {}

    #[when_closed(trigger = true, region = "CircuitRegion:Users:addr")]
    fn set_opened_for_addr(&self, env: &Env, addr: &Address) {}
}

// Oracle service module.
pub mod oracle_service {
    use soroban_kit::{oracle::Envelope, oracle::Events, oracle_broker};
    use soroban_sdk::{
        contract, contractimpl, contracttype, symbol_short, token, Address, Bytes, Env, Symbol, Vec,
    };

    const PAYMENT_TOKEN: Symbol = symbol_short!("TOKEN");

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Key {
        Synchronous,
    }

    #[contract]
    #[oracle_broker(Address, Bytes)]
    pub struct TestBrokerContract;

    #[contractimpl]
    impl TestBrokerContract {
        pub fn set_token(env: Env, token: Address) {
            env.storage().instance().set(&PAYMENT_TOKEN, &token);
        }
    }

    impl Events<Address, Bytes> for TestBrokerContract {
        fn on_subscribe(env: &Env, topic: &Address, envelope: &Envelope) -> Option<Bytes> {
            super::println!("on_subscribe {:?} {:?}", *topic, *envelope);

            // e.g., Take a fee from subscriber.
            envelope.subscriber.require_auth();
            let token_addr = env
                .storage()
                .instance()
                .get::<_, Address>(&PAYMENT_TOKEN)
                .unwrap();
            let token = token::Client::new(&env, &token_addr);
            token.transfer(
                &envelope.subscriber,
                &env.current_contract_address(),
                &10000000,
            );

            let mut envelopes = if env.storage().instance().has::<Address>(topic) {
                env.storage()
                    .instance()
                    .get::<Address, Vec<Envelope>>(topic)
                    .unwrap()
            } else {
                Vec::new(env)
            };

            // Support for synchronous messaging is possible by
            // returning the data directly on subscription.
            if env.storage().instance().has(&Key::Synchronous) {
                env.storage().instance().get::<_, Bytes>(&Key::Synchronous)
            } else {
                envelopes.push_back(envelope.clone());
                env.storage()
                    .instance()
                    .set::<Address, Vec<Envelope>>(topic, &envelopes);
                None
            }
        }

        fn on_publish(
            env: &Env,
            topic: &Address,
            data: &Bytes,
            _publisher: &Address,
        ) -> Vec<Envelope> {
            super::println!("on_publish {:?}", *topic);
            let envelopes = env
                .storage()
                .instance()
                .get::<Address, Vec<Envelope>>(topic)
                .unwrap();
            env.storage().instance().remove::<Address>(topic);
            env.storage().instance().set(&Key::Synchronous, data);
            envelopes
        }
    }
}

#[contract]
#[oracle_subscriber(Address, Bytes)]
pub struct TestContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OracleClientKey {
    Data,
}

impl Events<Address, Bytes> for TestContract {
    fn on_request(_env: &Env, _topic: &Address, envelope: &Envelope) {
        envelope.subscriber.require_auth();
    }

    fn on_async_receive(env: &Env, _topic: &Address, _envelope: &Envelope, data: &Bytes) {
        assert_eq!(*data, bytes!(&env, [1, 2, 3]));
        env.storage().instance().set(&OracleClientKey::Data, data);
    }
}

#[contractimpl]
impl TestContract {
    pub fn hello_circuit_breaker(env: Env) {
        let circuit = Circuit;
                
        // The circuit is closed by default, these should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.can_call_when_opened(&env);
            circuit.set_closed(&env);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect state"
        );

        circuit.can_call_when_closed(&env);
        circuit.set_opened(&env);

        // The circuit is now opened, these should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.can_call_when_closed(&env);
            circuit.set_opened(&env);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect state"
        );

        circuit.can_call_when_opened(&env);
        circuit.set_closed(&env);

        // The circuit is closed by default, these should panic.
        let addr = Address::generate(&env);

        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.can_call_when_opened_for_addr(&env, &addr);
            circuit.set_closed_for_addr(&env, &addr);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect state"
        );

        circuit.can_call_when_closed_for_addr(&env, &addr);
        circuit.set_opened_for_addr(&env, &addr);

        // The circuit is now opened, these should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            circuit.can_call_when_closed_for_addr(&env, &addr);
            circuit.set_opened_for_addr(&env, &addr);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect state"
        );

        circuit.can_call_when_opened_for_addr(&env, &addr);
        circuit.set_closed_for_addr(&env, &addr);
    }

    pub fn hello_commit_reveal(env: Env) {
        // This example demonstrates the use of a commitment scheme in a Rock-Paper-Scissors game smart contract.

        // Commitment schemes allow parties to commit to a value, keeping it hidden until a later time.
        // This technique can be applied in use cases such as voting systems, zero-knowledge proofs (ZKPs),
        // pseudo-random number generation (PRNG) seeding and more.

        // soroban-kit `commit` and `reveal` macros facilitate the implementation of the commitment scheme
        // in Soroban smart contracts.

        // Additionally, the soroban-kit `state_machine` macro is used to model the game's state transitions
        // with concurrency and extended states to allow players to play in any order.

        // First, we initialize two player accounts.
        let player_one = Player::Alice;
        let player_two = Player::Bob;

        // Each player prepares their data in private.
        // For enhanced security, it is recommended to hash the data with a random salt.

        let mut data_player_one = bytes!(&env, [1]);
        let salt_player_one = BytesN::from_array(&env, &[0_u8; 32]);
        data_player_one.append(&Bytes::from_slice(&env, &salt_player_one.to_array()));
        let hash_player_one = env.crypto().sha256(&data_player_one);

        let mut data_player_two = bytes!(&env, [2]);
        let salt_player_two = BytesN::from_array(&env, &[1_u8; 32]);
        data_player_two.append(&Bytes::from_slice(&env, &salt_player_two.to_array()));
        let hash_player_two = env.crypto().sha256(&data_player_two);

        // All set, let's play RockPaperScissors!
        let game = RockPaperScissors;

        // Set initial game state to the commit phase for each player.
        game.reset_player(&env, Player::Alice);
        game.reset_player(&env, Player::Bob);

        // Any attempt to play the reveal phase or solve should fail.
        let result = catch_unwind(AssertUnwindSafe(|| {
            game.reveal(&env, &player_one, &data_player_one);
            game.reveal(&env, &player_two, &data_player_one);
            game.solve(&env);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect phase"
        );

        // Players commit their hash.
        // Note that the order of play does not matter as the state machine
        // supports concurrency with regions.
        game.play(&env, &player_two, &hash_player_two);
        game.play(&env, &player_one, &hash_player_one);

        // Players should not be able to play again.
        // Both the `commit` and `state-machine` macros can prevent replay.
        let result = catch_unwind(AssertUnwindSafe(|| {
            game.play(&env, &player_one, &hash_player_one);
            game.play(&env, &player_two, &hash_player_two);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Incorrect phase"
        );

        // Players reveal their data.
        game.reveal(&env, &player_one, &data_player_one);
        game.reveal(&env, &player_two, &data_player_two);

        // All players data revealed, we can solve the game.
        game.solve(&env);
    }

    pub fn hello_state_machine(env: Env) {
        // soroban-kit FSM allows modeling complex concurrency behaviors
        // with regions and extended state variables.

        let buyer1 = Address::generate(&env);
        let buyer2 = Address::generate(&env);
        let coffee_machine = CoffeeMachine;

        // Set the initial state to Refilled.
        coffee_machine.refill(&env);

        // We can use regions to create a composite state (see #[state_machine] attribute).
        // so buyers can operate the machine concurrently.
        coffee_machine.insert_coin(&env, &buyer1);
        coffee_machine.insert_coin(&env, &buyer2);

        // We can use extended state variables to model the distribution
        // state for each item (Americano and Cappuccino).
        // The state machine transition will panic! if `distribute` is invoked
        // for an item the specific buyer did not select.
        coffee_machine.select(&env, &buyer1, &Item::Americano);
        coffee_machine.select(&env, &buyer2, &Item::Cappuccino);

        // Try to distribute Cappuccino to Buyer1
        let result = catch_unwind(AssertUnwindSafe(|| {
            coffee_machine.distribute(&env, &buyer1, &Item::Cappuccino);
        }));
        assert!(
            result.is_err(),
            "The operation should panic. Buyer1 selected Americano"
        );

        coffee_machine.distribute(&env, &buyer1, &Item::Americano);
        coffee_machine.distribute(&env, &buyer2, &Item::Cappuccino);
    }

    pub fn hello_storage(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        let key = Key::Newcomer;
        let data = Data { newcomer };

        // Try to get the newcomer from storage, should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(storage::get(&env, &key).unwrap(), data);
        }));
        assert!(result.is_err(), "None set. The operation should panic.");

        // Let's set it then.
        storage::set(&env, &key, &data);
        assert_eq!(storage::has(&env, &key), true);

        // Greetings from storage!
        vec![
            &env,
            symbol_short!("Hello"),
            storage::get(&env, &key).unwrap().newcomer,
        ]
    }

    pub fn hello_oracle(env: Env) -> Bytes {
        env.storage()
            .instance()
            .get::<_, Bytes>(&OracleClientKey::Data)
            .unwrap()
    }
}

#[test]
fn test_soroban_kit_hello_state_machine() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract)).hello_state_machine();
}

#[test]
fn test_soroban_kit_hello_storage() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract))
        .hello_storage(&symbol_short!("Fred"));
}

#[test]
fn test_soroban_kit_hello_commit_reveal() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract)).hello_commit_reveal();
}

#[test]
fn test_soroban_kit_hello_circuit_breaker() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract))
        .hello_circuit_breaker();
}

#[test]
fn test_oracle() {
    let env = Env::default();
    env.mock_all_auths();
    let broker = oracle_service::TestBrokerContractClient::new(
        &env,
        &env.register_contract(None, oracle_service::TestBrokerContract),
    );

    let topic = Address::generate(&env);

    let subscriber1 = Address::generate(&env);
    let subscriber2 = Address::generate(&env);
    let subscriber3 = Address::generate(&env);

    let (payment_token, payment_token_admin) =
        create_token_contract(&env, &Address::generate(&env));
    broker.set_token(&payment_token.address);

    payment_token_admin.mint(&subscriber1, &10000000000);
    payment_token_admin.mint(&subscriber2, &10000000000);
    payment_token_admin.mint(&subscriber3, &10000000000);

    let client1 = TestContractClient::new(&env, &env.register_contract(None, TestContract));
    let client2 = TestContractClient::new(&env, &env.register_contract(None, TestContract));
    let client3 = TestContractClient::new(&env, &env.register_contract(None, TestContract));

    // Data has not been published.
    assert_eq!(client1.request(&topic, &subscriber1, &broker.address), None);
    assert_eq!(client2.request(&topic, &subscriber2, &broker.address), None);

    let publisher = Address::generate(&env);
    broker.publish(&topic, &publisher, &bytes!(&env, [1, 2, 3]));

    // The data should now be available to the clients.
    assert_eq!(client1.hello_oracle(), bytes!(&env, [1, 2, 3]));
    assert_eq!(client2.hello_oracle(), bytes!(&env, [1, 2, 3]));

    // After publishing, clients should be able to get the
    // results synchronously from this oracle broker.
    assert_eq!(
        client3
            .request(&topic, &subscriber3, &broker.address)
            .unwrap(),
        bytes!(&env, [1, 2, 3])
    );
}
