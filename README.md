[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# soroban-kit
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)

Fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development. All modules in Rust crates are [feature flagged](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section), compile just what you need and nothing more!

- [soroban-kit](#soroban-kit)
  - [soroban-macros crate](#soroban-macros-crate)
    - [Storage Macros](#storage-macros)
      - [Background](#background)
      - [Usage](#usage)
  - [soroban-tools crate](#soroban-tools-crate)
    - [Storage Module](#storage-module)
      - [Usage](#usage-1)
    - [Testing notes](#testing-notes)
  - [hello-soroban-kit contract](#hello-soroban-kit-contract)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)

## soroban-macros crate

[Complete soroban-macros documentation](crates/soroban-macros/)

A collection of procedural macros designed to streamline development for Soroban smart contracts.

### Storage Macros

`storage` and `key_constraint` generate a minimal wrapper for type safety with storage operations while also enforcing type rules at compile time, binding Soroban storage, data types and keys. For performance, the generated code handles key and data operations without duplication, leveraging Rust lifetimes for safe borrowing.

#### Background

When dealing with the Soroban storage, repetitive boilerplate code is typically required for encapsulation and type safety over generic storage functions.

```rust
    // For all storage functions... copy-pasted for all custom data types...
    fn set_user_data(key: &userKey, data: &UserData)    
```

The `storage` macros streamline this process by automatically generating the boilerplate code, enforcing type rules at compile time, binding the storage with custom data types and optionally, applying Trait constraints to storage keys.

#### Usage

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
Now you have access to type-safe operations for managing CustomType with the Soroban instance storage.

```rust
    use soroban_tools::storage;

    // e.g., Set the data to instance storage.
    let data = CustomType { token: Address::random(&env) };
    let key = Key::User(Address::random(&env));      
    storage::set(&env, &key, &data);

    // e.g., Retrieve the data from instance storage.
    let data = storage::get(&env, &key);

    // e.g., Retrieve the data with a closure.
    let data = storage::get_or_else(&env, &key, |opt| opt.unwrap_or_else(|| default_value()));

    // e.g., Call has(), bump() and remove() with type inference syntax.
    storage::has::<Key, CustomType>(&env, &key);
    storage::bump::<Key, CustomType>(&env, &key, 1, 1);
    storage::remove::<Key, CustomType>(&env, &key);
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

## soroban-tools crate

[Complete soroban-tools documentation](crates/soroban-tools/)

### Storage Module

The `storage` module exports the `impl_storage!` and `impl_key_constraint!` macros which are essentially the declarative versions of the procedural macros exported from the `soroban-macros` crate.

#### Usage

Detailed usage and documentation can be found in the `soroban-tools` [README.md](crates/soroban-tools/README.md).


### Testing notes

1. You can run tests with mock-storage feature enabled outside of Soroban environment:
   ```sh
   cargo test --features mock-storage
   ```

## hello-soroban-kit contract

[Complete hello-soroban-kit documentation](crates/hello-soroban-kit/)

A simple Soroban smart contract example showcasing the use of `soroban-tools` and `soroban-macros`. 

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