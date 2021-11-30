use crate::{mock::*, Error, ContractEntry};
use frame_support::{assert_ok, weights::Weight};
use sp_runtime::traits::Hash;

#[test]
fn it_accepts_calls_from_chain_extension() {
	let origin = Origin::signed(ALICE);
	let chain_extension_input = 5;
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TemplateModule::insert_number(origin, chain_extension_input));
		assert_eq!(ContractEntry::<Test>::get(), chain_extension_input);
	})
}
