# Substrate Runtime and Contract Interactions

A Substrate node demonstrating interactions between the runtime and Ink! smart contracts.

| :exclamation: This code is not audited or considered ready for production, and should not be used as such. |
| ---------------------------------------------------------------------------------------------------------- |

## Prerequisites

If you have not already, it is recommended to go through the [ink! smart contracts tutorial](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/) or otherwise have writted and compiled smart contracts according to the [ink! docs](https://paritytech.github.io/ink-docs/).

Cargo Contracts is required to run the smart contracts tests.

```
# For Ubuntu or Debian users
sudo apt install binaryen
# For MacOS users
brew install binaryen

cargo install cargo-contract --vers ^0.15 --force --locked
```

## Motivation

Examples of contract-to-runtime interactions are frequently sought and asked about. `Chain extensions` are recommended for this case, and there are [several examples of them available](https://paritytech.github.io/ink-docs/macros-attributes/chain-extension). A full example of interactions going in both directions has not been developed in a recent Substrate version.

### Contract-to-Runtime Interactions

The project demonstrates contract-to-runtime interactions through the use of Chain extensions. Chain Extensions allow a runtime developer to extend runtime functions to smart contracts. The project uses Chain Extensions to make available a custom pallet function to smart contracts. See also the `rand-extension` chain extension code example, which is one example that this project _extended_.

### Runtime-to-Contract Interactions

Runtime-to-contract interactions are enabled through invocations of the pallet-contract's own `bare_call` method, invoked from a custom pallet extrinsic. The extrinsic is called `call_smart_contract` and is meant to demonstrate calling an existing(uploaded and instantiated) smart-contract generically. The caller specifies the account id of the smart contract to be called, the selector of the smart contract function(this is found in the metadata.json in the compiled contract), and one argument to be passed to the smart contract function.

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Testing

To run the tests for the included example pallet, run `cargo test`, in the root.
To run the tests for the smart contract example, run `cargo +nightly contract test`, or `cargo +nightly test` within the `smart-contracts/example-extension` directory.

### Benchmarks

Build node with benchmarks enabled:

`cargo build --release --features runtime-benchmarks`

Then, to generate the weights into the pallet template's `weights.rs` file:

```zsh
./target/release/node-template benchmark \
 --chain dev \
 --pallet=pallet_template \
 --extrinsic='\*' \
 --repeat=20 \
 --steps=50 \
 --execution wasm \
 --wasm-execution compiled \
 --raw \
 --output pallets/template/src/weights.rs \
 --template=./weight-template.hbs
```
