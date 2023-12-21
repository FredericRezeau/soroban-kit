/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

/// Integration tests for the commitment-scheme macros module (`commit`).
///
/// This test demonstrates the use of the commitment scheme to implement a polling station
/// in a smart contract with soroban-kit `commit` and `reveal` macros.
///
/// Additionally, soroban-kit `state_machine` macro is used to model the state transitions
/// with concurrency and extended states to allow multiple voters to vote in any order.
#[cfg(feature = "commitment-scheme")]
mod tests {

    extern crate soroban_tools;
    extern crate std;

    use core::panic::AssertUnwindSafe;
    use soroban_sdk::{
        bytes, contract, contractimpl, contracttype, symbol_short, vec, Bytes, BytesN, Env, Symbol,
        Vec,
    };

    use soroban_macros::{commit, key_constraint, reveal, state_machine, storage};
    use soroban_tools::{
        fsm::{self, StateMachine, TransitionHandler},
        storage,
    };

    use std::panic::catch_unwind;

    macro_rules! create_voters {
        ($($variant:ident),+) => {
            #[key_constraint(VoterConstraint)]
            #[contracttype]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum Voter {
                $($variant),+
            }
            impl Voter {
                pub fn all(env: &Env) -> Vec<Voter> {
                    vec![env, $(Voter::$variant),+]
                }
            }
        };
    }
    create_voters!(Alice, Bob, Charlie);

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Phase {
        Opened,
        Committing(Voter),
        Revealing(Voter),
        Completed(Voter),
        Closed,
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Domain {
        Booth(Voter),
        Station,
    }

    #[storage(Instance, VoterConstraint)]
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RevealedVote {
        data: Bytes,
    }

    pub struct PollingStation;

    impl PollingStation {
        #[commit]
        #[state_machine(state = "Phase:Committing:voter", region = "Domain:Booth:voter")]
        fn vote(&self, env: &Env, voter: &Voter, hash: &BytesN<32>) -> Symbol {
            // The vote commitment has completed successfully.
            // Voter will be allowed to reveal once all other voters have commited.
            symbol_short!("Voted")
        }

        #[reveal]
        #[state_machine(state = "Phase:Revealing:voter", region = "Domain:Booth:voter")]
        fn reveal(&self, env: &Env, voter: &Voter, data: &Bytes) {
            // Save the revealed data for the counting phase.
            storage::set(env, voter, &RevealedVote { data: data.clone() });
        }

        #[state_machine(state = "Phase:Closed", region = "Domain:Station")]
        fn count(&self, env: &Env) {
            // Iterate through all voters...
            let voters = Voter::all(env);
            for voter in voters.iter() {
                println!("Result: {:?} voted {:?}", voter, storage::get(env, &voter));
            }
        }

        fn register_voter(&self, env: &Env, voter: Voter) {
            let domain = Domain::Booth(voter.clone());
            let phase = Phase::Committing(voter.clone());
            let state_machine =
                StateMachine::<Domain, Phase>::new(&domain, fsm::StorageType::Instance);
            state_machine.set_state(&env, &phase);
        }
    }

    // Handle the polling station phase transitions.
    impl TransitionHandler<Domain, Phase> for PollingStation {
        fn on_effect(&self, env: &Env, state_machine: &StateMachine<Domain, Phase>) {
            let domain = state_machine.get_region();
            let phase = state_machine.get_state(&env).unwrap();
            let voters = Voter::all(env);
            if let Domain::Booth(current_voter) = domain {
                if let Some(voter) = voters.iter().find(|v| v == current_voter) {
                    match phase {
                        Phase::Committing(_) => {
                            // Voted, transition to revealing phase.
                            state_machine.set_state(&env, &Phase::Revealing(voter));
                        }
                        Phase::Revealing(_) => {
                            // Revealed, transition to completed phase.
                            state_machine.set_state(&env, &Phase::Completed(voter));

                            // Close the polling station if all voted.
                            let all_voted = voters.iter().all(|voter| {
                                let region = Domain::Booth(voter);
                                let sm = StateMachine::<Domain, Phase>::new(
                                    &region,
                                    fsm::StorageType::Instance,
                                );
                                matches!(
                                    sm.get_state(env).unwrap_or(Phase::Opened),
                                    Phase::Completed(_)
                                )
                            });
                            if all_voted {
                                let sm = StateMachine::<Domain, Phase>::new(
                                    &Domain::Station,
                                    fsm::StorageType::Instance,
                                );
                                sm.set_state(env, &Phase::Closed);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        fn on_guard(&self, _env: &Env, _state_machine: &StateMachine<Domain, Phase>) {
            // Left as an exercise for the reader:
            // e.g., implement a time-based guard to restrict the amount of time
            // allowed for voters...
        }
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Ballot {
        voter: Voter,
        data: Bytes,
        salt: BytesN<32>,
        hash: BytesN<32>,
    }

    impl Ballot {
        // Each voter prepares their vote in private.
        // For enhanced security, it is recommended to hash the data with a random salt (not implemented here).
        fn simulate_secret_vote(env: &Env, voter: Voter, vote_value: u8) -> Self {
            let mut data = bytes!(&env, [vote_value]);
            let salt = BytesN::from_array(&env, &[0_u8; 32]);
            data.append(&Bytes::from_slice(&env, &salt.to_array()));
            let hash = env.crypto().sha256(&data);
            Ballot {
                voter,
                data,
                salt,
                hash,
            }
        }
    }

    #[contract]
    pub struct TestContract;

    #[contractimpl]
    impl TestContract {
        pub fn test_commit_reveal(env: Env) {
            // Commitment schemes allow parties to commit to a value, keeping it hidden until a later time.
            // This technique can be applied in use cases such as voting systems, zero-knowledge proofs (ZKPs),
            // pseudo-random number generation (PRNG) seeding and more.

            let polling_station = PollingStation;

            let ballots = vec![
                &env,
                Ballot::simulate_secret_vote(&env, Voter::Alice, 2),
                Ballot::simulate_secret_vote(&env, Voter::Bob, 1),
                Ballot::simulate_secret_vote(&env, Voter::Charlie, 3),
            ];

            // Register the voters.
            for ballot in ballots.iter() {
                polling_station.register_voter(&env, ballot.voter);
            }

            // Try to reveal or count votes (should fail).
            let result = catch_unwind(AssertUnwindSafe(|| {
                for ballot in ballots.iter() {
                    polling_station.reveal(&env, &ballot.voter, &ballot.data);
                }
                polling_station.count(&env);
            }));
            assert!(
                result.is_err(),
                "The operation should panic. Incorrect phase"
            );

            // Voters commit their hash.
            // Note that the vote order does not matter, the state machine
            // handles concurrency for this phase.
            for ballot in ballots.iter() {
                polling_station.vote(&env, &ballot.voter, &ballot.hash);
            }

            // Voters should not be able to vote again.
            let result = catch_unwind(AssertUnwindSafe(|| {
                for ballot in ballots.iter() {
                    polling_station.vote(&env, &ballot.voter, &ballot.hash);
                }
            }));
            assert!(
                result.is_err(),
                "The operation should panic. Incorrect phase"
            );

            // Voters reveal their vote.
            for ballot in ballots.iter() {
                polling_station.reveal(&env, &ballot.voter, &ballot.data);
            }

            // All votes revealed, the polling station should have transitioned to closed.
            // we can start counting...
            polling_station.count(&env);
        }
    }

    #[test]
    fn test_macros_commit_reveal() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_commit_reveal();
    }
}
