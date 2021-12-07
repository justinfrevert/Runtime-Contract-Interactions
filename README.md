# Substrate Runtime and Contract Interactions

## A Substrate node demonstrating two-way interactions between the runtime and Ink! smart contracts.

| :exclamation: This code is not audited or considered ready for production, and should not be used in a production-like environment without any necessary review and changes |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |

## Motivation

Sharing Substrate runtime functionality with ink! smart contracts is a powerful feature. Chains with unique runtime functionality can create rich application developer ecosystems by exposing choice pieces of their runtime. The inverse interaction of runtime to ink! smart contract calls may be similarly valuable. Runtime logic can query or set important context information at the smart contracts level.

Both of the types of interactions described above are asked about in the context of support, and a recent example demonstrating how to perform these interactions has not been developed. This project demonstrates through example how to perform interactions in both directions, through an extrinsic call, and an ink! chain extension.

## Prerequisites

If you have not already, it is recommended to go through the [ink! smart contracts tutorial](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/) or otherwise have written and compiled smart contracts according to the [ink! docs](https://paritytech.github.io/ink-docs/). It is also recommended to have some experience with [Substrate runtime development](https://docs.substrate.io/v3/getting-started/overview/).

Ensure you have

1. Installed Substrate according to the [instructions](https://docs.substrate.io/v3/getting-started/installation/)
2. Run:

```sh
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

2. Installed Cargo Contracts

```sh
# For Ubuntu or Debian users
sudo apt install binaryen
# For MacOS users
brew install binaryen

cargo install cargo-contract --vers ^0.15 --force --locked
```

### Contract-to-Runtime Interactions

The project demonstrates contract-to-runtime interactions through the use of Chain extensions. Chain Extensions allow a runtime developer to extend runtime functions to smart contracts. In the case of this example, the functions being extended are a custom pallet extrinsic, and the `pallet_balances::transfer` extrinsic.

See also the `rand-extension` chain extension code example, which is one example that this project _extended_.

### Runtime-to-Contract Interactions

Runtime-to-contract interactions are enabled through invocations of the pallet-contract's own `bare_call` method, invoked from a custom pallet extrinsic. The example extrinsic is called `call_smart_contract` and is meant to demonstrate calling an existing(uploaded and instantiated) smart-contract generically. The caller specifies the account id of the smart contract to be called, the selector of the smart contract function(found in the metadata.json in the compiled contract), and one argument to be passed to the smart contract function.

### Build

#### Node

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

#### Smart contracts

To build the included smart contract example, first `cd` into `smart-contracts/example-extension`. then run:

```sh
cargo +nightly contracts build
```

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Local Contract Deployment

Once the smart contract is compiled, you may use the [hosted Canvas UI](https://paritytech.github.io/canvas-ui/#/). Please follow the [Deploy Your Contract guide](https://paritytech.github.io/ink-docs/getting-started/deploy-your-contract/) for specific instructions. This contract uses a `default` constructor, so there is no need to specify values for its constructor.

You may also use the [Polkadotjs Apps UI](https://polkadot.js.org/apps/#/contracts) to upload and instantiate the contract.

### Example Usage

Ensure you have uploaded and instantiated the example contract.

#### **Pallet-to-contract**

_Call the `set_value` smart contract function from a generic pallet extrinsic_

1. Browse to [extrinsics](https://polkadot.js.org/apps/#/extrinsics) in the Polkadotjs apps UI.
2. Supply the necessary arguments to instruct our extrinsic to call the smart contract function.
   Enter the following values in the `Submission` tab:
   - **dest**: AccountId of the desired contract.
   - **submit the following extrinsic** : `templateModule`
   - **selector**: `0x00abcdef` (note: this denotes the function to call, and is found in `smart-contracts/example-extension/target/ink/metadata.json`. See more [here](https://paritytech.github.io/ink-docs/macros-attributes/selector) on the ink! selector macro)
   - **arg**: some `u32` of your choice
   - **gasLimit**: `10000000000`
3. `Submit Transaction` -> `Sign and Submit`.

This extrinsic passed these arguments to the pallet_contracts::bare_call function, which resulted in our `set_value` smart contract function being called with the new `u32` value. This value can now be verified by calling the `get_value`, and checking whether the new value is returned.

#### **Contract-to-pallet**

_Call the `insert_number` extrinsic from the smart contract_

1. Browse to the [Execute page in the hosted Canvas UI](https://paritytech.github.io/canvas-ui/#/execute)
2. Under `chain-extension-example`, click `Execute`.
3. Under `Message to Send`, select `store_in_runtime`.
4. Enter some `u32` to be stored.
5. Ensure `send as transaction` is selected.
6. Click `Call`

The smart contract function is less generic than the extrinsic used above, and so aready knows how to call our custom runtime extrinsic through the chain extension that is set up. You can verify that the contract called the extrinsic by checking the `contractEntry` storage in the Polkadotjs UI.

### Testing

To run the tests for the included example pallet, run `cargo test` in the root.

### Benchmarks

Build node with benchmarks enabled:

`cargo build --release --features runtime-benchmarks`

Then, to generate the weights into the pallet template's `weights.rs` file:

```sh
./target/release/node-template benchmark \
 --chain dev \
 --pallet=pallet_template \
 --extrinsic='*' \
 --repeat=20 \
 --steps=50 \
 --execution wasm \
 --wasm-execution compiled \
 --raw \
 --output pallets/template/src/weights.rs \
 --template=./weight-template.hbs
```
