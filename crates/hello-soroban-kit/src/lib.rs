/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#![no_std]

mod examples; // Examples module.
mod types; // Contract types.

use examples::{example_circuit_breaker, example_rock_paper_scissors, example_storage};

use soroban_kit::{oracle, oracle_subscriber, storage};
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol, Vec};

use types::{Message, MessageKey, Whitelist, WhitelistKey};

pub trait HelloContractTrait {
    fn circuit_breaker(env: Env) -> Symbol;
    fn rock_paper_scissors(env: Env) -> Symbol;
    fn hello_oracle(env: Env, topic: Bytes) -> Message;
    fn whitelist_broker(env: Env, broker: Address);
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol>;
}

#[contract]
// Implement the Oracle Subscriber interface for the contract.
// We use `bytes` (native type) for the topic and a custom type `Message`
// for the data.
#[oracle_subscriber(Bytes, Message)]
pub struct HelloContract;

// Implement the Oracle events.
impl oracle::Events<Bytes, Message> for HelloContract {
    // This event fires before the call to oracle broker subscribe().
    fn on_request(env: &Env, _topic: &Bytes, envelope: &oracle::Envelope) {
        // Example: whitelist brokers allowed to interact with your contract.
        assert_eq!(
            storage::get(&env, &WhitelistKey::Broker).unwrap().broker,
            envelope.broker
        );
        envelope.subscriber.require_auth();
    }

    // This event fires when the data is received synchronously.
    fn on_sync_receive(env: &Env, topic: &Bytes, envelope: &oracle::Envelope, data: &Message) {
        assert_eq!(
            storage::get(&env, &WhitelistKey::Broker).unwrap().broker,
            envelope.broker
        );
        storage::set(&env, &MessageKey::Topic(topic.clone()), &data);
    }

    // This event fires when the data is received asynchronously.
    fn on_async_receive(env: &Env, topic: &Bytes, envelope: &oracle::Envelope, data: &Message) {
        // Only allow whitelisted broker.
        assert_eq!(
            storage::get(&env, &WhitelistKey::Broker).unwrap().broker,
            envelope.broker
        );
        // Make sure the broker is authorized (i.e., made the cross-contract call).
        envelope.broker.require_auth();
        // Set the data.
        storage::set(&env, &MessageKey::Topic(topic.clone()), &data);
    }
}

#[contractimpl]
impl HelloContractTrait for HelloContract {
    // Example implementing a pausable activity with
    // soroban-kit `circuit-breaker`, `when_opened` and `when_closed` attr. macros.
    fn circuit_breaker(env: Env) -> Symbol {
        example_circuit_breaker::hello(env)
    }

    // Example implementing "rock paper scissors" with
    // soroban-kit `state-machine`, `commit` and `reveal` attr. macros.
    fn rock_paper_scissors(env: Env) -> Symbol {
        example_rock_paper_scissors::hello(env)
    }

    // Example retrieving a message received from an oracle.
    fn hello_oracle(env: Env, topic: Bytes) -> Message {
        storage::get(&env, &MessageKey::Topic(topic)).unwrap()
    }

    fn whitelist_broker(env: Env, broker: Address) {
        assert!(!storage::has(&env, &WhitelistKey::Broker));
        storage::set(&env, &WhitelistKey::Broker, &Whitelist { broker });
    }

    // Example saying hello with soroban-kit type safe `storage` attr. macros.
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        example_storage::hello(env, newcomer)
    }
}

#[cfg(test)]
mod test;
