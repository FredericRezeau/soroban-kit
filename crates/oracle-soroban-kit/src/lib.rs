/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

// This example implements a simple Oracle Broker to handle synchronous and asynchronous
// topic-based requests for a fee.

#![no_std]

use soroban_kit::{oracle, oracle_broker};
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Bytes, Env, TryIntoVal, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub data: Bytes,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Database {
    Topic(Bytes),
}

#[contract]
// In this example, we use `Bytes` (native type) for the topic and a custom type
// `Message` for the data. All built-in and user defined types are supported.
#[oracle_broker(Bytes, Message)]
pub struct OracleContract;

// Implement the Oracle events trait.
impl oracle::Events<Bytes, Message> for OracleContract {
    // This event is fired for each data requests.
    fn on_subscribe(env: &Env, topic: &Bytes, envelope: &oracle::Envelope) -> Option<Message> {
        // Retrieve the envelopes for this topic.
        let mut envelopes = if env.storage().instance().has::<Bytes>(topic) {
            env.storage()
                .instance()
                .get::<Bytes, Vec<oracle::Envelope>>(topic)
                .unwrap()
        } else {
            Vec::new(env)
        };

        // Here, you typically handle authentication,
        // apply filters for routers, subscribers, topics,
        // other checks as needed.

        // Example: topic-based filter.
        // assert_eq!(*topic, bytes![env, [1, 2, 3]]);

        // Example: Enforce max envelopes per topics.
        // assert!(envelopes.len() < 5);

        // In this example, we demonstrate how to charge a fee
        // for all subscriber requests.
        envelope.subscriber.require_auth();
        token::Client::new(
            &env,
            &Address::from_string(
                &"CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
                    .try_into_val(env)
                    .unwrap(),
            ),
        )
        .transfer(
            &envelope.subscriber,
            &env.current_contract_address(),
            &10000000,
        );

        // Return the data synchronously if available.
        if env
            .storage()
            .instance()
            .has(&Database::Topic(topic.clone()))
        {
            env.storage()
                .instance()
                .get::<_, Message>(&Database::Topic(topic.clone()))
        } else {
            // Otherwise, add the envelope for asynchronous publishing.
            envelopes.push_back(envelope.clone());
            env.storage()
                .instance()
                .set::<Bytes, Vec<oracle::Envelope>>(topic, &envelopes);
            None
        }
    }

    // This event is fired for each publishing requests.
    fn on_publish(
        env: &Env,
        topic: &Bytes,
        data: &Message,
        _publisher: &Address,
    ) -> Vec<oracle::Envelope> {
        // Here, you typically handle authentication for publishers
        // other checks as needed.

        // Store the data for synchronous requests.
        env.storage()
            .instance()
            .set::<_, Message>(&Database::Topic(topic.clone()), data);

        // In this example, we simply return all envelopes @topic
        let envelopes = env
            .storage()
            .instance()
            .get::<Bytes, Vec<oracle::Envelope>>(topic)
            .unwrap();
        env.storage().instance().remove::<Bytes>(topic);
        envelopes
    }
}

// That's it! Ready to deploy your Oracle broker contract!
