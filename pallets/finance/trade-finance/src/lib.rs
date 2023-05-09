#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchResult},
	pallet_prelude::MaxEncodedLen,
	traits::{Get},
	RuntimeDebug,
};


mod tests;

mod mock;

#[derive(
Clone,	Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
)]
pub struct LetterOfCredit {
    id: u64,
    trade_id: u64,
    amount: u64,
    beneficiary: u64,
    issuing_bank: u64,
    status: u64,
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;


	}

	#[pallet::storage]
	#[pallet::getter(fn get_lc)]
	//  declaring LetterOfCredit as storage item:
	pub type LetterOfCreditStore<T> = StorageValue<_, LetterOfCredit>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LetterOfCreditStored { lc: LetterOfCredit, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn send_lc(origin: OriginFor<T>, lc: LetterOfCredit) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let lc_clone: LetterOfCredit = lc.clone();
			// Update storage.
			<LetterOfCreditStore<T>>::put(lc_clone);

			// Emit an event.
			Self::deposit_event(Event::LetterOfCreditStored { lc, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
/* 
		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <LetterOfCreditStore<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<LetterOfCreditStore<T>>::put(new);
					Ok(())
				},
			}
		}
*/
	}
}
