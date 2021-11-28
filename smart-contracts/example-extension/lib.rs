#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{AccountId, Environment};
use ink_lang as ink;

#[ink::chain_extension]
pub trait MyExtension {
	type ErrorCode = ContractError;
	// Specify the function id. We will `match` on this in the runtime to map this to some custom
	// pallet extrinsic
	#[ink(extension = 1)]
	fn do_store_in_runtime(key: u32) -> Result<u32, ContractError>;
	#[ink(extension = 2)]
	fn do_balance_transfer(value: u32, recipient: AccountId) -> Result<u32, ContractError>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
	FailToCallRuntime,
}

impl From<scale::Error> for ContractError {
	fn from(_: scale::Error) -> Self {
		panic!("encountered unexpected invalid SCALE encoding")
	}
}

impl ink_env::chain_extension::FromStatusCode for ContractError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::FailToCallRuntime),
			_ => panic!("encountered unknown status code"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
	const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

	type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
	type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
	type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
	type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
	type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
	// type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;

	type ChainExtension = MyExtension;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod runtime_extension {
	use super::ContractError;

	/// Defines the storage of our contract.
	#[ink(storage)]
	pub struct RuntimeInterface {
		stored_number: u32,
	}

	#[ink(event)]
	pub struct UpdatedNum {
		result: u32,
	}

	// impl for functions that demonstrate two way communication between runtime and smart contract
	impl RuntimeInterface {
		#[ink(constructor)]
		pub fn default() -> Self {
			Self { stored_number: Default::default() }
		}

		/// Get value from runtime storage, by the current calling address
		#[ink(message)]
		pub fn get_value(&mut self) -> u32 {
			self.stored_number
		}

		/// A simple storage function meant to demonstrate calling a smart contract with an argument
		/// from a custom pallet
		#[ink(message, selector = 0xABCDEF)]
		pub fn set_value(&mut self, value: u32) -> Result<(), ContractError> {
			self.stored_number = value;
			self.env().emit_event(UpdatedNum { result: value });
			Ok(())
		}

		/// Invoke the extended custom pallet extrinsic with the argument given to the smart
		/// contract funtion
		#[ink(message)]
		pub fn store_in_runtime(&mut self, value: u32) -> Result<(), ContractError> {
			self.env().extension().do_store_in_runtime(value)?;
			self.env().emit_event(UpdatedNum { result: value });
			Ok(())
		}

		// invoke the extended transfer function with the arguments given to the smart contract
		// function
		#[ink(message)]
		pub fn extended__transfer(
			&mut self,
			amount: u32,
			recipient: AccountId,
		) -> Result<(), ContractError> {
			self.env().extension().do_balance_transfer(amount, recipient)?;
			self.env().emit_event(UpdatedNum { result: amount });
			Ok(())
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;
		use ink_lang as ink;

		const RUNTIME_STORE_FN_ID: u32 = 1;
		const EXTENDED_TRANSFER_FN_ID: u32 = 2;

		#[ink::test]
		fn set_value_works() {
			let mut test_api = RuntimeInterface::default();
			test_api.set_value(5);
			assert_eq!(test_api.get_value(), 5);
		}

		// chain extension setup for tests. You can use different chain extension functions within
		// one test by passing new `expected_func_id` for other extension
		macro_rules! setup_test_chain_extension {
			($extension_name:ident, $expected_func_id:expr) => {
				impl ink_env::test::ChainExtension for $extension_name {
					/// The static function id of the chain extension.
					fn func_id(&self) -> u32 {
						$expected_func_id
					}

					/// The chain extension is called with the given input.
					///
					/// Returns an error code and may fill the `output` buffer with a
					/// SCALE encoded result. The error code is taken from the
					/// `ink_env::chain_extension::FromStatusCode` implementation for
					/// `ContractError`.
					fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
						let ret: [u8; 32] = [1; 32];
						scale::Encode::encode_to(&ret, output);
						0
					}
				}
				ink_env::test::register_chain_extension($extension_name);
			};
		}

		#[ink::test]
		fn extended_set_and_get_emits_event() {
			struct MockedExtension;
			setup_test_chain_extension!(MockedExtension, RUNTIME_STORE_FN_ID);
			let mut test_api = RuntimeInterface::default();

			test_api.store_in_runtime(5);

			let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
			// Ensure the method that calls the chain extension emitted an event
			assert_eq!(emitted_events.len(), 1);
		}

		#[ink::test]
		fn extended_transfer_emits_event() {
			struct MockedExtension;
			setup_test_chain_extension!(MockedExtension, EXTENDED_TRANSFER_FN_ID);

			let mut test_api = RuntimeInterface::default();
			let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()?;

			test_api.extended__transfer(5, accounts.bob);

			let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
			// Ensure the method that calls the chain extension emitted an event
			assert_eq!(emitted_events.len(), 1);
		}
	}
}
