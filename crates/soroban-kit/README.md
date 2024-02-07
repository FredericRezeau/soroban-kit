[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-kit
![Build Status](https://github.com/FredericRezeau/soroban-kit/actions/workflows/rust.yml/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/soroban-kit.svg)](https://crates.io/crates/soroban-kit)

> Fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. 

`soroban-kit` is designed for compactness, focusing on slim constructs. It is built on Rust's dependency-free `core` library and the `soroban-sdk`. All modules are [feature-flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), allowing you to compile just what you need and nothing more!

Licensed under MIT. This software is provided "AS IS", no liability assumed. [More details](LICENSE).

- [soroban-kit](#soroban-kit)
  - [Features](#features)
    - [Extended State Machine](#extended-state-machine)
      - [Background](#background)
      - [Documentation](#documentation)
      - [Examples](#examples)
    - [Oracle](#oracle)
      - [Background](#background-1)
      - [Documentation](#documentation-1)
      - [Examples](#examples-1)
    - [Commitment Scheme](#commitment-scheme)
      - [Background](#background-2)
      - [Documentation](#documentation-2)
      - [Examples](#examples-2)
    - [Circuit Breaker](#circuit-breaker)
      - [Background](#background-3)
      - [Documentation](#documentation-3)
      - [Examples](#examples-3)
    - [Type Safe Storage](#type-safe-storage)
      - [Background](#background-4)
      - [Documentation](#documentation-4)
      - [Examples](#examples-4)
    - [Utils](#utils)
  - [Smart Contract Demo](#smart-contract-demo)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)


## Features

### Extended State Machine

```toml
[dependencies]
soroban-kit = { version = "0.1.10", default-features = false, features = ["state-machine"] }
```

The `state_machine` attribute macro can be used to implement versatile state machines (see [fsm/impl.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-tools/src/fsm/impl.rs)) in Soroban smart contracts. It features state concurrency through regions (composite states), runtime behavior modeling via extended state variables, transition control with guards and effects, and state persistence with Soroban storage.

#### Background

While state machines are a prevalent behavioral pattern in Solidity smart contracts, their implementation is often limited due to Solidity rigid architecture leading to complexities, and sometimes impossibilities, in implementing concurrency and runtime behaviors.

Leveraging Rust advanced type system, soroban-kit `state_machine` can handle complex interactions and concurrent state executions, enabling a flexible, yet straightforward state machine solution for Soroban smart contracts.

#### Documentation

Configure a function for state transition within your finite state machine.

`#[state_machine]` options:
- `state`: StatePath := EnumName ":" VariantName [":" TupleVariableName]
- `region`: RegionPath := EnumName ":" VariantName [":" TupleVariableName]
- `storage`: "instance" (default) | "persistent" | "temporary"
```rust
    // Example
    #[state_machine(
      state = "Phase:Committing:voter",
      region = "Domain:Booth:voter")]
    fn my_state_machine_function(&self, env: &Env, voter: &Voter) {
    }
```

Use the `TransitionHandler` trait to control state transitions with guards and effects.

```rust
#[derive(TransitionHandler)]
pub struct MyStateMachine;

impl MyStateMachine {
    // Implement to provide guard conditions for the transition
    // (e.g., ledger sequence or time-based guards).
    fn on_guard(/* omitted parameters */) {}

    // Implement the effect from transitioning.
    fn on_effect(/* omitted parameters */) {}
}
```

#### Examples

- [Polling Station Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs)
- [Game Lobby Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/state-machine-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Oracle

The `oracle_broker` and `oracle_subscriber` attribute macros are designed to generalize interfacing for both asynchronous and synchronous cross-contract communication. Leveraging the publisher-subscriber pattern, the system allows subscribers to establish multiple connections to oracle broker contracts and *vice-versa*.

These macros generate a lightweight framework (see [oracle.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/src/oracle.rs)) ensuring consistency for communication and events-driven interactions (see [impl.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-tools/src/oracle/impl.rs)). `topic` and `data` types are customizable via macro arguments, you can use any user-defined and built-in Soroban types.

#### Background

Oracles serve as bridges between blockchains and external data sources. There are many key challenges in implementing Oracle services, including decentralization, synchronicity, decoupling and multiplicity.

`soroban-kit` proposes a lightweight solution for implementing the *pub/sub* messaging pattern to help address these challenges for cross-contract communication.

See the [oracle-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/oracle-soroban-kit) contract for a basic oracle broker implementation showcasing fee collection from subscribers, with the ability to serve data both synchronously and asynchronously based on availability.

#### Documentation

`#[oracle_broker]` options (positional arguments):
- `Topic Type`: Built-in or Custom type
- `Data Type`: Built-in or Custom type
```rust
    // Implement the oracle broker interface for your contract.
    #[contract]
    #[oracle_broker(Bytes, MyDataType)]
    pub struct OracleContract;
```

`#[oracle_subscriber]` options (positional arguments):
- `Topic Type`: Built-in or Custom type
- `Data Type`: Built-in or Custom type
```rust
    // Implement the oracle subscriber interface for your contract.
    #[contract]
    #[oracle_subscriber(Bytes, MyDataType)]
    pub struct TestContract;
```

The framework allows you to handle various events via the `oracle:Events` trait implementation:

```rust
    // Example, receiving data asynchronously.
    fn on_async_receive(env: &Env, topic: &Bytes, envelope: &oracle::Envelope, data: &Message) {
        // Only allow whitelisted oracle broker.
        assert_eq!(
            storage::get(&env, &WhitelistKey::Broker).unwrap().broker,
            envelope.broker
        );
        // Make sure the broker is authorized (i.e., made the cross-contract call).
        envelope.broker.require_auth();
        // Set the data.
        storage::set(&env, &MessageKey::Topic(topic.clone()), &data);
    }
```

#### Examples

- [oracle-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/oracle-soroban-kit)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Commitment Scheme

```toml
[dependencies]
soroban-kit = { version = "0.1.10", default-features = false, features = ["commitment-scheme"] }
```
The `commit` and `reveal` attribute macros are designed to easily implement the commitment scheme in your Soroban smart contract. They use the soroban-sdk _sha256_ or _keccak256_ hash functions and storage with automatic hash removal.

These attributes can also be paired with the `state_machine` attribute to manage the commitment and reveal phases for multiple parties. For a comprehensive demo of such pairing, refer to the [Polling Station](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs) example.

```rust
        #[commit]
        #[state_machine(state = "Phase:Committing")]
        fn vote(&self, env: &Env, hash: &BytesN<32>) {
        }

        #[reveal]
        #[state_machine(state = "Phase:Revealing")]
        fn reveal(&self, env: &Env, data: &Bytes) {
        }
```

#### Background

Commitment schemes allow parties to commit to a value, keeping it hidden until a later time. This technique can be applied in use cases such as voting systems, zero-knowledge proofs (ZKPs), pseudo-random number generation (PRNG) seeding and more.

The `commit` and `reveal` macros in `soroban-kit` allow a boilerplate-free implementation of the commitment scheme using rust attributes.

#### Documentation

`#[commit]` options:
- `hash`: VariableName (default = "hash")
- `storage`: "instance" (default) | "persistent" | "temporary"
```rust
    // Example
    #[commit]
    fn my_commit_function(env: &Env, hash: &BytesN<32>) {
    }
```

`#[reveal]` options:
- `data`: VariableName (default = "data")
- `hash_func`: "sha256" (default) | "keccak256"
- `clear_commit`: true (default) | false
- `storage`: "instance" (default) | "persistent" | "temporary"
```rust
    // Example
    #[reveal]
    fn my_reveal_function(env: &Env, data: &Bytes) {
    }
```

#### Examples

- [Polling Station Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Circuit Breaker

```toml
[dependencies]
soroban-kit = { version = "0.1.10", default-features = false, features = ["circuit-breaker"] }
```

The `when_opened` and `when_closed` attribute macros provide a streamlined way to integrate the circuit breaker pattern into your Soroban smart contracts.

These macros, also leveraging the `state-machine` module, enable detailed control over state transitions (see [circuit_breaker.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/src/circuit_breaker.rs)) and the construction of composite circuits (i.e., grouping operations in sub circuits).

#### Background

In the context of smart contracts, the circuit breaker pattern serves as a vital security mechanism, safeguarding stakeholders in the event of unexpected contract behavior or external attacks. This pattern is prevalent in Solidity smart contracts, notably through the popular [Pausable contract](https://docs.openzeppelin.com/contracts/2.x/api/lifecycle) module from OpenZeppelin.

`soroban-kit` macros allow a straightforward implementation of the circuit-breaker pattern for any operation and subset of operations in your contract.


#### Documentation

`#[when_opened]` / `#[when_closed]` options:
- `region`: RegionPath := EnumName ":" VariantName [":" TupleVariableName]
- `trigger`: A boolean to indicate if the function call should trigger a state change (default: false).

```rust
    #[derive(CircuitBreaker)]
    struct Circuit; 
    
    impl Circuit {
        // bid() is usable when the circuit is closed (operational).
        #[when_closed]
        fn bid(&self, env: &Env) {
        }

        // emergency_stop() triggers a state change.
        #[when_closed(trigger = true)]
        fn emergency_stop(&self, env: &Env) {
        }

        // upgrade() also restores bid() operation.
        #[when_opened(trigger = true)]
        fn upgrade(&self, env: &Env) {
          // e.g., upgrade contract.
        }
    }
```

Control state transitions with guards and effects.

```rust
    impl Circuit {
        // Define guard conditions for state transitions (open/close).
        fn on_guard(/* omitted parameters */) {}

        // Define effects of state transitions
        fn on_effect(/* omitted parameters */) {}
    }
```

#### Examples

- [Circuit Breaker Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit/src/examples/example_circuit_breaker.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Type Safe Storage

```toml
[dependencies]
soroban-kit = { version = "0.1.10", default-features = false, features = ["storage"] }
```

The `storage` and `key_constraint` macros generate a minimal wrapper (see [storage/impl.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-tools/src/storage/impl.rs)) for type safety with storage operations while also enforcing type rules at compile time, binding Soroban storage, data types and keys. For performance, the generated code handles key and data operations without duplication, leveraging Rust lifetimes for safe borrowing.

#### Background

When dealing with the Soroban storage, repetitive boilerplate code is typically required for encapsulation and type safety over generic storage functions.

The `storage` macros streamline this process by automatically generating the boilerplate code, enforcing type rules at compile time, binding the storage with custom data types and optionally, applying Trait constraints to storage keys.

#### Documentation

`#[storage]` options (positional arguments):
- `Storage`: Instance (default) | Persistent | Temporary
- `Key`: Trait
```rust
    // Example
    #[storage(Instance, AdminKeyConstraint)]
    pub struct AdminData {
        pub address: Address,
    }
```

`#[key-constraint]` options (positional arguments):
- `Key`: Trait
```rust
    // Example
    #[key_constraint(AdminKeyConstraint)]
    pub enum Key {
        Admin,
    }
```

#### Examples

- [Walkthrough Video](https://www.youtube.com/watch?v=YZbI0MnyskE)
- [Integration Tests](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/storage-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Utils

```toml
[dependencies]
soroban-kit = { version = "0.1.10", default-features = false, features = ["utils"] }
```

This module contains utility macros including:

-  `reflective_enum`: enables *C-style* enums to reflect their own variants.

## Smart Contract Demo

`hello-soroban-kit` is a simple Soroban smart contract demo showcasing the use of all `soroban-kit` features. Read [hello-soroban-kit documentation](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit).

## Contributing

Contributions are welcome! If you have a suggestion that would make this better, please fork the repo and create a pull request.

## License

`soroban-kit` is licensed under the MIT License. See [LICENSE](LICENSE) for more details.

## Contact

For inquiries or collaborations:

Fred Kyung-jin Rezeau - [@FredericRezeau](https://twitter.com/fredericrezeau)

[license-shield]: https://img.shields.io/github/license/FredericRezeau/soroban-kit.svg?style=for-the-badge
[license-url]: https://github.com/FredericRezeau/soroban-kit/blob/master/LICENSE
[twitter-shield]: https://img.shields.io/badge/-Twitter-black.svg?style=for-the-badge&logo=twitter&colorB=555
[twitter-url]: https://twitter.com/fredericrezeau