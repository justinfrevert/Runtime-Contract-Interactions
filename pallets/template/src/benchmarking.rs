//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::{ContractEntry, Pallet as Template};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::UncheckedFrom;

benchmarks! {
	where_clause {
		where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>,
	 }
	insert_number {
		let s in 0 .. 4294967295;
		let caller: T::AccountId = whitelisted_caller();
	}: _ (RawOrigin::Signed(caller.clone()), s)
	verify {
		assert_eq!(ContractEntry::<T>::get(), s);
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
