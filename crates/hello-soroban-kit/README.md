[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# hello-soroban-kit
![Build Status](https://github.com/FredericRezeau/soroban-kit/actions/workflows/rust.yml/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/soroban-kit.svg)](https://crates.io/crates/soroban-kit)

This crate is part of `soroban-kit`: [Github](https://github.com/FredericRezeau/soroban-kit) | [crates.io](https://crates.io/crates/soroban-kit).

`hello-soroban-kit` is a Soroban smart contract demo showcasing the use of all features in `soroban-kit` including `state-machine`, `commitment-scheme`, `circuit-breaker`, `oracle` and `storage`.

`soroban-kit` is designed for compactness, focusing on slim constructs. It is built on Rust's dependency-free `core` library and the `soroban-sdk`. All modules are feature-gated, offering you the flexibility to compile only the components essential for your project.

Take a look at [Litemint Smart Contracts](https://github.com/litemint/litemint-soroban-contracts) to see an integration of the library in real-world smart contracts.

## Commands

1. Building the Contract:
   ```sh
   soroban contract build
   ```
2. Running Tests:
   ```sh
   cargo test -- --nocapture
   ```
3. Deploying to Testnet:
   
   ```sh
   soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_soroban_kit.wasm --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" --source ACCOUNT
   ```
   ```sh
   output > CONTRACT_ID
   ```
4. Invoking the contract:
   
   Just say hello with storage type-safety!
   ```sh
   soroban contract invoke --id CONTRACT_ID --source ACCOUNT --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" -- hello --newcomer TESTER
   ```
   ```sh
   output > ["Hello","TESTER"]
   ```

   Play rock paper scissors!
   ```sh
   soroban contract invoke --id CONTRACT_ID --source ACCOUNT --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" -- rock_paper_scissors
   ```
   ```sh
   output > "Success"
   ```

   Flip the switch on a circuit!
   ```sh
   soroban contract invoke --id CONTRACT_ID --source ACCOUNT --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" -- circuit_breaker
   ```
   ```sh
   output > "Success"
   ```
5. Check out [lib.rs](src/lib.rs) and [test.rs](src/test.rs) for detailed integration examples.
   

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