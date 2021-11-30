#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{AccountId, Environment};
use ink_lang as ink;

#[cfg(test)]
mod tests;

#[ink::chain_extension]
pub trait MyExtension {
	type ErrorCode = ContractError;
	// Use the #[ink(extension = {func_id})] syntax to specify the function id.
    // We will `match` on this in the runtime to map this to some custom pallet extrinsic
	#[ink(extension = 1)]
    /// Calls func_id 1, defined in the runtime, which receives a number and stores it in
    /// a runtime storage value
	fn do_store_in_runtime(key: u32) -> Result<u32, ContractError>;
	#[ink(extension = 2)]
    /// Calls func_id 2, which uses pallet_balances::transfer to perform a transfer of
    /// `value` from the sender, to `recipient`
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
/// A smart contract with a custom environment, necessary for the chain extension
mod contract_with_extension {
	use super::ContractError;

	#[cfg(test)]
	use crate::tests;

	/// Defines the storage of our contract.
	#[ink(storage)]
	pub struct RuntimeInterface {
		stored_number: u32,
	}

	#[ink(event)]
	pub struct ResultNum {
		number: u32,
	}

	// impl for functions that demonstrate two way communication between runtime and smart contract
	impl RuntimeInterface {
		#[ink(constructor)]
		pub fn default() -> Self {
			Self { stored_number: Default::default() }
		}

		/// Get currently stored value
		#[ink(message)]
		pub fn get_value(&mut self) -> u32 {
            self.env().emit_event(ResultNum { number: self.stored_number.clone() });
            self.stored_number
		}

		/// A simple storage function meant to demonstrate calling a smart contract from a custom pallet.
        /// Here, we've set the `selector` explicitly to make it simpler to target this function.
		#[ink(message, selector = 0xABCDEF)]
		pub fn set_value(&mut self, value: u32) -> Result<(), ContractError> {
			self.stored_number = value;
			self.env().emit_event(ResultNum { number: value });
			Ok(())
		}

		/// Invoke the extended custom pallet extrinsic with the argument given to the smart
		/// contract function
		#[ink(message)]
		pub fn store_in_runtime(&mut self, value: u32) -> Result<(), ContractError> {
			self.env().extension().do_store_in_runtime(value)?;
			self.env().emit_event(ResultNum { number: value });
			Ok(())
		}

		// Invoke the extended transfer function with the arguments given to the smart contract
		// function
		#[ink(message)]
		pub fn extended_transfer(
			&mut self,
			amount: u32,
			recipient: AccountId,
		) -> Result<(), ContractError> {
			self.env().extension().do_balance_transfer(amount, recipient)?;
			self.env().emit_event(ResultNum { number: amount });
			Ok(())
		}
	}
}
