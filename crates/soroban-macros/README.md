[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-macros
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)

This crate is part of [soroban-kit](https://github.com/FredericRezeau/soroban-kit) which provides fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. All modules in Rust crates are [feature flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), compile just what you need and nothing more!

`soroban-macros` provides a collection of procedural macros designed to streamline development for Soroban smart contracts.

## Storage Macros

`storage` and `key_constraint` generate a minimal wrapper for type safety with storage operations while also enforcing type rules at compile time, binding Soroban storage, data types and keys. For performance, the generated code handles key and data operations without duplication, leveraging Rust lifetimes for safe borrowing.

### Background

When dealing with the Soroban storage, repetitive boilerplate code is typically required for encapsulation and type safety over generic storage functions.

```rust
    // For all storage functions... copy-pasted for all custom data types...
    fn set_user_data(key: &userKey, data: &UserData)    
```

The `storage` macros streamline this process by automatically generating the boilerplate code, enforcing type rules at compile time, binding the storage with custom data types and optionally, applying Trait constraints to storage keys.

### Usage

Cargo.toml:
```toml
[dependencies]
soroban-macros = { version = "0.1.1", features = ["storage"] }
```

Bind the Soroban Instance storage to your custom contract type.

```rust
    use soroban_macros::{storage, key_constraint};
 
    // Implement the desired storage for your custom contract type.
    #[contracttype]
    #[storage(Instance)] // e.g., Instance storage binding.
    pub struct CustomType {
        pub token: Address,
    }
```
Now you have access to type-safe operations for managing CustomType with the Soroban instance storage via `soroban-tools` (also part of the `soroban-kit`).

```rust
    use soroban_tools::storage;

    // e.g., Retrieve the data from instance storage.
    let data = storage::get(&env, &key);
```
Optional. `key_constraint` allows you to express compile-time restrictions ensuring that only specific keys can be used with a storage-type binding:

```rust
    // Set up AdminKeyConstraint for AdminKey
    #[contracttype]  
    #[key_constraint(AdminKeyConstraint)]
    pub enum AdminKey {
        Admin,
    }

    // Apply it to the AdminData x Instance binding.
    #[contracttype]
    #[storage(Instance, AdminKeyConstraint)]
    pub struct AdminData {
        pub address: Address,
    }

    // The compiler will generate an error if the code attempts to use
    // AdminData storage with a key that does not satisfy AdminKeyConstraint.
```

## Contributing

Contributions are welcome! If you have a suggestion that would make this better, please fork the repo and create a pull request.

## License

`soroban-macros` part of `soroban-kit` is licensed under the MIT License. See [LICENSE](LICENSE) for more details.


## Contact

For inquiries or collaborations:

Fred Kyung-jin Rezeau - [@FredericRezeau](https://twitter.com/fredericrezeau)

[license-shield]: https://img.shields.io/github/license/FredericRezeau/soroban-kit.svg?style=for-the-badge
[license-url]: https://github.com/FredericRezeau/soroban-kit/blob/master/LICENSE
[twitter-shield]: https://img.shields.io/badge/-Twitter-black.svg?style=for-the-badge&logo=twitter&colorB=555
[twitter-url]: https://twitter.com/fredericrezeau