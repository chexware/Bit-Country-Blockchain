#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as StakingModule {
		
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn stake(origin, stake_balance: u32) -> dispatch::DispatchResult {
			let user = ensure_signed(origin)?;
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn unstake(origin, unstake_balance: u32) -> dispatch::DispatchResult {
			let user = ensure_signed(origin)?;
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn claim_rewards(origin) -> dispatch::DispatchResult {
			let user = ensure_signed(origin)?;
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn reinvest_rewards(origin) -> dispatch::DispatchResult {
			let user = ensure_signed(origin)?;
			Ok(())
		}
    }
}