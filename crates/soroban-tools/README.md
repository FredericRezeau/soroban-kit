[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-tools
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)

This crate is part of [soroban-kit](https://github.com/FredericRezeau/soroban-kit) which provides fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. All modules in Rust crates are [feature flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), compile just what you need and nothing more!

`soroban-tools` provide a collection of tools, functions and macros designed to streamline development for Soroban smart contracts.

- [soroban-tools](#soroban-tools)
  - [Modules](#modules)
    - [State Machine](#state-machine)
      - [Background](#background)
      - [Usage](#usage)
    - [Storage](#storage)
      - [Background](#background-1)
      - [Usage](#usage-1)
      - [Testing notes](#testing-notes)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)

## Modules

### State Machine

The `fsm` (finite state machine) module exports the `impl_state_machine!` declarative macro which can be used to implement versatile state machines in Soroban smart contracts. It features state concurrency through regions, runtime behavior modeling via extended state variables, transition control with guards and effects, and state persistence with Soroban storage.

#### Background

While state machines are a prevalent behavioral pattern in Solidity smart contracts, their implementation is often limited due to Solidity rigid architecture leading to complexities, and sometimes impossibilities, in implementing concurrency and runtime behaviors.

Leveraging Rust advanced type system, soroban-kit `impl_state_machine!` can handle complex interactions and concurrent state executions, enabling a flexible, yet straightforward state machine solution for Soroban smart contracts.

#### Usage

```rust
    // Example.
    soroban_tools::impl_state_machine!(
        env,
        fsm::StorageType::Instance,
        State, Idle, (), Region, All, ()
    );
```

Check out the integration tests [Gaming Lobby](/crates/soroban-macros/tests/state-machine-tests.rs) and [Coffee Machine](/crates/hello-soroban-kit/src/test.rs) state machine examples for detailed usage.

Cargo.toml:
```toml
[dependencies]
soroban-tools = { version = "0.1.2", features = ["state-machine"] }
```

### Storage

The `storage` module exports the `impl_storage!` and `impl_key_constraint!` declarative macros which generate a minimal wrapper for type safety with storage operations while also enforcing type rules at compile time, binding Soroban storage, data types and keys. For performance, the generated code handles key and data operations without duplication, leveraging Rust lifetimes for safe borrowing.

#### Background

When dealing with the Soroban storage, repetitive boilerplate code is typically required for encapsulation and type safety over generic storage functions.

```rust
    // For all storage functions... copy-pasted for all custom data types...
    fn set_user_data(key: &userKey, data: &UserData)    
```

The `storage` macros streamline this process by automatically generating the boilerplate code, enforcing type rules at compile time, binding the storage with custom data types and optionally, applying Trait constraints to storage keys.

#### Usage

```rust
     #[contracttype]
     pub enum Key {
         User(Address),
     }

     #[contracttype]
      pub struct CustomType {
          pub token: Address,
      }
 
      // Use the `impl_storage!` macro to implement the desired storage
      // for any custom contract type.
      // e.g., Bind the Soroban instance storage to CustomType.
      impl_storage!(Instance, CustomType); 
```

Optional. The `impl_key_constraint!` macro allows you to express compile-time restrictions ensuring that only specific key traits can be used with a storage-type binding:

```rust
      // e.g., The following implements an AdminKeyConstraint trait to Admin key
      // then applies it to the storage.
      impl_key_constraint!(AdminKey, AdminKeyConstraint);
      impl_storage!(Instance, AdminData, AdminKeyConstraint);

      // The compiler will now generate an error if the Key type used with the AdminData
      // storage does not satisfy AdminKeyConstraint.
```

Check out the [Storage](/crates/soroban-macros/tests/state-machine-tests.rs) integration tests and [Hello](/crates/hello-soroban-kit/src/test.rs) smart contract examples for detailed usage.

Cargo.toml:
```toml
[dependencies]
soroban-tools = { version = "0.1.2", features = ["storage"] }
```
#### Testing notes

1. You can run tests with mock-storage feature enabled outside of Soroban environment:
   ```sh
   cargo test --features mock-storage
   ```

## Contributing

Contributions are welcome! If you have a suggestion that would make this better, please fork the repo and create a pull request.

## License

`soroban-tools` part of `soroban-kit` is licensed under the MIT License. See [LICENSE](LICENSE) for more details.


## Contact

For inquiries or collaborations:

Fred Kyung-jin Rezeau - [@FredericRezeau](https://twitter.com/fredericrezeau)

[license-shield]: https://img.shields.io/github/license/FredericRezeau/soroban-kit.svg?style=for-the-badge
[license-url]: https://github.com/FredericRezeau/soroban-kit/blob/master/LICENSE
[twitter-shield]: https://img.shields.io/badge/-Twitter-black.svg?style=for-the-badge&logo=twitter&colorB=555
[twitter-url]: https://twitter.com/fredericrezeau