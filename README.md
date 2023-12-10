[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-kit
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)

Fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. All modules in Rust crates are [feature flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), compile just what you need and nothing more!

- [soroban-kit](#soroban-kit)
  - [soroban-macros crate](#soroban-macros-crate)
    - [State Machine Macro](#state-machine-macro)
      - [Background](#background)
      - [Documentation](#documentation)
    - [Storage Macros](#storage-macros)
      - [Background](#background-1)
      - [Documentation](#documentation-1)
  - [soroban-tools crate](#soroban-tools-crate)
    - [State Machine Module](#state-machine-module)
      - [Documentation](#documentation-2)
    - [Storage Module](#storage-module)
      - [Documentation](#documentation-3)
  - [hello-soroban-kit contract](#hello-soroban-kit-contract)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)

## soroban-macros crate

A collection of procedural macros designed to streamline development for Soroban smart contracts. Read the [documentation](crates/soroban-macros/).

### State Machine Macro

The `state-machine` attribute macro can be used to implement versatile state machines in Soroban smart contracts. It features state concurrency through regions, runtime behavior modeling via extended state variables, transition control with guards and effects, and state persistence with Soroban storage.

#### Background

While state machines are a prevalent behavioral pattern in Solidity smart contracts, their implementation is often limited due to Solidity rigid architecture leading to complexities, and sometimes impossibilities, in implementing concurrency and runtime behaviors.

Leveraging Rust advanced type system, soroban-kit `state-machine` can handle complex interactions and concurrent state executions, enabling a flexible, yet straightforward state machine solution for Soroban smart contracts.

#### Documentation

Make sure you check out the [Gaming Lobby](/crates/soroban-macros/tests/state-machine-tests.rs) and [Coffee Machine](/crates/hello-soroban-kit/src/test.rs) state machines examples. Complete documentation can be found in the `soroban-macros` [README](crates/soroban-macros/README.md).

### Storage Macros

`storage` and `key_constraint` generate a minimal wrapper for type safety with storage operations while also enforcing type rules at compile time, binding Soroban storage, data types and keys. For performance, the generated code handles key and data operations without duplication, leveraging Rust lifetimes for safe borrowing.

#### Background

When dealing with the Soroban storage, repetitive boilerplate code is typically required for encapsulation and type safety over generic storage functions.

```rust
    // For all storage functions... copy-pasted for all custom data types...
    fn set_user_data(key: &userKey, data: &UserData)    
```

The `storage` macros streamline this process by automatically generating the boilerplate code, enforcing type rules at compile time, binding the storage with custom data types and optionally, applying Trait constraints to storage keys.

#### Documentation

Documentation and usage examples can be found in the `soroban-macros` [README](crates/soroban-macros/README.md).

## soroban-tools crate

### State Machine Module

The `fsm` (finite state machine) module exports the `impl_state_machine!` macro which is essentially the declarative version of the procedural macro exported from the `soroban-macros` crate.

#### Documentation

Documentation and usage examples can be found in the `soroban-tools` [README](crates/soroban-tools/README.md).

### Storage Module

The `storage` module exports the `impl_storage!` and `impl_key_constraint!` macros which are essentially the declarative versions of the procedural macros exported from the `soroban-macros` crate.

#### Documentation

Documentation and usage examples can be found in the `soroban-tools` [README](crates/soroban-tools/README.md).


## hello-soroban-kit contract

A simple Soroban smart contract example showcasing the use of `soroban-tools` and `soroban-macros`.

Read [hello-soroban-kit documentation](crates/hello-soroban-kit/).

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