[![MIT License][license-shield]][license-url]
[![Twitter][twitter-shield]][twitter-url]

# hello-soroban-kit
[![Build Status](https://app.travis-ci.com/FredericRezeau/soroban-kit.svg?branch=main)](https://app.travis-ci.com/FredericRezeau/soroban-kit)

This crate is part of [soroban-kit](https://github.com/FredericRezeau/soroban-kit) which provides fast, lightweight functions and macros with lean, targeted functionality for Soroban smart contract development.

`hello-soroban-kit` is a simple Soroban smart contract example showcasing the use of `soroban-tools` and `soroban-macros`.

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
   
   ```sh
   soroban contract invoke --id CONTRACT_ID --source ACCOUNT --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" -- hello --newcomer TESTER
   ```
   ```sh
   output > ["Hello","TESTER"]
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