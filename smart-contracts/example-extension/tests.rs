/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
#[cfg(test)]
pub mod tests {
	/// Imports all the definitions from the outer scope so we can use them here.
	use super::*;
	use crate::contract_with_extension::RuntimeInterface;
	use ink_lang as ink;

	// Existing function ids, for convenience
	const RUNTIME_STORE_FN_ID: u32 = 1;
	const EXTENDED_TRANSFER_FN_ID: u32 = 2;
	const EXTENDED_BALANCE_FN_ID: u32 = 3;

	// Mock return values
	const UNUSED_RETURN: u32 = 0;
	const TRANSFER_AMOUNT: u32 = 400;

	#[ink::test]
	fn set_value_works() {
		let mut test_api = RuntimeInterface::default();
		test_api.set_value(5);
		assert_eq!(test_api.get_value(), 5);
	}

	// chain extension setup for tests. You can use different chain extension functions within
	// one test by passing new `expected_func_id` for other extension
	macro_rules! mock_chain_extension {
		($extension_name:ident, $expected_func_id:expr, $mock_return:expr) => {
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
					let ret= $mock_return;
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
		let mocked_return = 0;
		mock_chain_extension!(MockedExtension, RUNTIME_STORE_FN_ID, UNUSED_RETURN);
		let mut test_api = RuntimeInterface::default();

		test_api.store_in_runtime(5);

		let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
		assert_eq!(emitted_events.len(), 1);
	}

	#[ink::test]
	fn extended_transfer_works() {
		struct ExtensionForTransfer;
		struct ExtensionForBalanceCheck;

		mock_chain_extension!(ExtensionForTransfer, EXTENDED_TRANSFER_FN_ID, UNUSED_RETURN);

		let mut test_api = RuntimeInterface::default();
		let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()?;

		test_api.extended_transfer(5, accounts.bob);

		// register new mock chain extension in context of test to use different extended function
		mock_chain_extension!(ExtensionForBalanceCheck, EXTENDED_BALANCE_FN_ID, TRANSFER_AMOUNT);

		// get balance from balances pallet for django into recent event
		let result = test_api.get_balance(accounts.django);
		assert_eq!(result.ok().unwrap(), TRANSFER_AMOUNT);
	}
}
