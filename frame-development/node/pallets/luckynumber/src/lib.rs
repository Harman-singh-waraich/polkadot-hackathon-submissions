#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get,traits::Vec};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct MyLuckyNumber {
    number: u32,
    whyLucky  : Vec<u8>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
decl_storage! {
	trait Store for Module<T: Trait> as LuckyNumber {
		LuckyNumber get(fn lucky_number_by_account):
			map hasher(blake2_128_concat) T::AccountId => MyLuckyNumber;
	}
}

// Pallets use events to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]

		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		#[weight = 50_000_000]
		fn set_lucky_number(origin,number: u32, why: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let num = MyLuckyNumber{
				number,
				whyLucky: why,
			};

			<LuckyNumber<T>>::insert(&sender, num);
			Ok(())
		}
	}
}
