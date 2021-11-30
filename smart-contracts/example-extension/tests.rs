/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
#[cfg(test)]
pub mod tests {
	/// Imports all the definitions from the outer scope so we can use them here.
	use super::*;
	use crate::runtime_extension::RuntimeInterface;
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

		test_api.extended_transfer(5, accounts.bob);

		let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
		// Ensure the method that calls the chain extension emitted an event
		assert_eq!(emitted_events.len(), 1);
	}
}
