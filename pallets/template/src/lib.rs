#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult, inherent::Vec, pallet_prelude::*, traits::Currency,
	};
	use frame_system::pallet_prelude::*;
	use pallet_contracts::chain_extension::UncheckedFrom;

	type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_contracts::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	pub const MAX_LENGTH: usize = 50;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_items)]
	pub(super) type ContractEntry<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CalledContract(T::AccountId),
		CalledPalletFromContract(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ArgumentTooLarge,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>,
	{
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// A generic example to demonstrate calling a smart contract from an extrinsic
		pub fn call_smart_contract(
			origin: OriginFor<T>,
			dest: T::AccountId,
			// selector as given in the metadata.json file of the compiled contract
			mut selector: Vec<u8>,
			arg: u32,
			// gas_limit should be set somewhere ~10000000000
			#[pallet::compact] gas_limit: Weight,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(selector.len() < MAX_LENGTH, Error::<T>::ArgumentTooLarge);
			// Amount to transfer
			let value: BalanceOf<T> = Default::default();
			let mut arg_enc: Vec<u8> = arg.encode();
			let mut data = Vec::new();
			data.append(&mut selector);
			data.append(&mut arg_enc);

			pallet_contracts::Pallet::<T>::bare_call(
				who,
				dest.clone(),
				value,
				gas_limit,
				data,
				false,
			)
			.result?;

			Self::deposit_event(Event::CalledContract(dest.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// An extrinsic for demonstrating calls originating from a smart contract
		pub fn insert_number(origin: OriginFor<T>, val: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Do something with the value
			ContractEntry::<T>::insert(who, val);
			Self::deposit_event(Event::CalledPalletFromContract(val));
			Ok(())
		}
	}
}
