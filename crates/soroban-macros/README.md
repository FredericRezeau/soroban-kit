[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-macros
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)
[![Current Crates.io Version](https://img.shields.io/crates/v/soroban-kit.svg)](https://crates.io/crates/soroban-kit)

This crate is part of `soroban-kit`: [Github](https://github.com/FredericRezeau/soroban-kit) | [crates.io](https://crates.io/crates/soroban-kit).

Fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. All modules are [feature flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), compile just what you need and nothing more!

- [soroban-macros](#soroban-macros)
  - [Features](#features)
    - [Extended State Machine](#extended-state-machine)
      - [Background](#background)
      - [Documentation](#documentation)
      - [Examples](#examples)
    - [Commitment scheme](#commitment-scheme)
      - [Background](#background-1)
      - [Documentation](#documentation-1)
      - [Examples](#examples-1)
    - [Type Safe Storage](#type-safe-storage)
      - [Background](#background-2)
      - [Documentation](#documentation-2)
      - [Examples](#examples-2)
  - [Smart Contract Demo](#smart-contract-demo)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)


## Features

### Extended State Machine

```toml
[dependencies]
soroban-kit = { version = "0.1.4", default-features = false, features = ["state-machine"] }
```

The `state-machine` attribute macro can be used to implement versatile state machines (see [fsm/impl.rs](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-tools/src/fsm/impl.rs)) in Soroban smart contracts. It features state concurrency through regions (composite states), runtime behavior modeling via extended state variables, transition control with guards and effects, and state persistence with Soroban storage.

#### Background

While state machines are a prevalent behavioral pattern in Solidity smart contracts, their implementation is often limited due to Solidity rigid architecture leading to complexities, and sometimes impossibilities, in implementing concurrency and runtime behaviors.

Leveraging Rust advanced type system, soroban-kit `state-machine` can handle complex interactions and concurrent state executions, enabling a flexible, yet straightforward state machine solution for Soroban smart contracts.

#### Documentation

`#[state-machine]` options:
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

#### Examples

- [Polling Station Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs)
- [Game Lobby Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/state-machine-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Commitment scheme

```toml
[dependencies]
soroban-kit = { version = "0.1.4", default-features = false, features = ["commitment-scheme"] }
```
The `commit` and `reveal` attribute macros are designed to easily implement the commitment scheme in your Soroban smart contract. They use the soroban-sdk _sha256_ or _keccak256_ hash functions and storage with automatic hash removal.

These attributes can also be paired with the `state-machine` attribute to manage the commitment and reveal phases for multiple parties. For a comprehensive demo of such pairing, refer to the [Polling Station](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs) example.

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
    #[commit(hash = "commit_hash")]
    fn my_commit_function(env: &Env, commit_hash: &BytesN<32>) {
    }
```

`#[reveal]` options:
- `data`: VariableName (default = "data")
- `hash_func`: "sha256" (default) | "keccak256"
- `clear_commit`: true (default) | false
- `storage`: "instance" (default) | "persistent" | "temporary"
```rust
    // Example
    #[reveal(data = "reveal_data")]
    fn my_reveal_function(env: &Env, reveal_data: &Bytes) {
    }
```

#### Examples

- [Polling Station Example](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/commit-reveal-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

### Type Safe Storage

```toml
[dependencies]
soroban-kit = { version = "0.1.4", default-features = false, features = ["storage"] }
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

- [Integration Tests](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/soroban-macros/tests/storage-tests.rs)
- [hello-soroban-kit](https://github.com/FredericRezeau/soroban-kit/blob/master/crates/hello-soroban-kit)

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