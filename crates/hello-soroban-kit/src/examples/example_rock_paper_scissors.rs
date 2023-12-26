/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

// This example demonstrates the use of a commitment scheme in a Rock-Paper-Scissors game smart contract.

// For comprehensive commit-reveal example check:
// - Polling station example: `crates/soroban-macros/tests/commit-reveal-tests.rs`

// Commitment schemes allow parties to commit to a value, keeping it hidden until a later time.
// This technique can be applied in use cases such as voting systems, zero-knowledge proofs (ZKPs),
// pseudo-random number generation (PRNG) seeding and more.

// soroban-kit `commit` and `reveal` macros facilitate the implementation of the commitment scheme
// in Soroban smart contracts.

// Additionally, the soroban-kit `state-machine` macro is used to model the game's state transitions
// with concurrency and extended states to allow players to play in any order.

use soroban_sdk::{bytes, symbol_short, Bytes, BytesN, Env, Symbol,};

use soroban_kit::{
    commit,
    fsm::{StateMachine, StorageType},
    reveal, soroban_tools, state_machine, TransitionHandler,
};

use crate::types::{Domain, Phase, Player};

#[derive(TransitionHandler)]
pub struct RockPaperScissors;

impl RockPaperScissors {
    // TransitionHandler::on_effect
    // Called immediately after state validation iff validation succeeded.
    // Used to implement the effect from transitioning.
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
                StateMachine::<Domain, Phase>::new(&Domain::Game, StorageType::Instance);
            let next_phase = match state_machine.get_state(env).unwrap_or(Phase::Start) {
                Phase::Completed(player) if player == check_player => Phase::End,
                _ => Phase::Completed(for_player),
            };
            state_machine.set_state(env, &next_phase);
        }
    }

    // TransitionHandler::on_guard
    // Called immediately before state validation.
    // Used to implement guard conditions for the transition (e.g., ledger sequence or time-based guards).
    fn on_guard(&self, _env: &Env, _state_machine: &StateMachine<Domain, Phase>) {
        // Here you could implement a time-based guard to restrict the amount of time
        // allowed for players and so on...
    }

    // Commit phase.
    #[commit]
    #[state_machine(state = "Phase:Committing:player", region = "Domain:Players:player")]
    fn play(&self, env: &Env, player: &Player, hash: &BytesN<32>) {}

    // Reveal phase.
    #[reveal]
    #[state_machine(state = "Phase:Revealing:player", region = "Domain:Players:player")]
    fn reveal(&self, env: &Env, player: &Player, data: &Bytes) {}

    // Solve phase.
    #[state_machine(state = "Phase:End", region = "Domain:Game")]
    fn solve(&self, env: &Env) -> Symbol {
        symbol_short!("Success")
    }

    fn reset_player(&self, env: &Env, player: Player) {
        let domain = Domain::Players(player.clone());
        let phase = Phase::Committing(player.clone());
        let state_machine = StateMachine::<Domain, Phase>::new(&domain, StorageType::Instance);
        state_machine.set_state(&env, &phase);
    }
}

pub fn hello(env: Env) -> Symbol {
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

    // Players commit their hash.
    // Note that the order of play does not matter as the state machine
    // supports concurrency with regions.
    game.play(&env, &player_two, &hash_player_two);
    game.play(&env, &player_one, &hash_player_one);

    // Players reveal their data.
    game.reveal(&env, &player_one, &data_player_one);
    game.reveal(&env, &player_two, &data_player_two);

    // All players data revealed, we can solve the game.
    game.solve(&env)
}
