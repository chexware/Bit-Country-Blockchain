#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, 
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
	IterableStorageDoubleMap, Parameter,
	debug,
};

use codec::{Encode, Decode};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One, Zero},
    DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::{self as system, ensure_signed};
use primitives::*;



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, Clone, RuntimeDebug)]
pub enum RewardType {
	Inflationary,
	Deflationary,
}

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, Clone, RuntimeDebug)]
pub enum RewardsDestination {
	Staker,
  	Account(AccountId),
  	None
}

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug)]
pub struct ActiveEraInfo {
	/// Index of era.
	pub index: u64,
	/// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
	///
	/// Start can be none if start hasn't been set for the era yet,
	/// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
	start: Option<u64>,
}

type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: 
frame_system::Config
+ pallet_balances::Config {
	type Event: From<Event<Self>> 
	+ Into<<Self as frame_system::Config>::Event>;
	
	type BalanceLockPeriod: Get<Self::BlockNumber>;

	type EraLength: Get<Self::BlockNumber>;

    /// Type for EraId 
	type EraId: Parameter
    + Member
    + AtLeast32BitUnsigned
    + Default
    + Copy
    + MaybeSerializeDeserialize
    + Bounded;

	/// Type for RewardMultiplier
	type RewardMultiplier: Parameter
    + Member
    + AtLeast32BitUnsigned
    + Default
    + Copy
    + MaybeSerializeDeserialize
    + Bounded;

	type Currency: Currency<Self::AccountId>;
}

// The pallet's runtime storage items.
decl_storage! {
	trait Store for Module<T: Config> as StakingModule {
		// Store all staking rewards payout account
		pub PayoutAccounts get(fn get_payout_account): map hasher(twox_64_concat) T::AccountId => Option<T::AccountId>;
		// Store all staking rewards for an account
		pub RewardBalances get(fn get_reward_balance): map hasher(twox_64_concat) T::AccountId => Option<T::Balance>;
		// Store all staked balances
		pub StakedBalances get(fn get_staked_balance): map hasher(twox_64_concat) T::AccountId => Option<T::Balance>;
		// Store all locked balances
		pub LockedBalances get(fn get_locked_balance): map hasher(twox_64_concat) T::AccountId => Option<T::Balance>;
		// Track era
		pub EraIndex get(fn era_index): T::EraId;
		// Store era reward multiplier
		pub EraReward get(fn get_era_reward): map hasher(twox_64_concat) T::EraId => T::RewardMultiplier;

	}
}

decl_event!(
	pub enum Event<T> where 
	<T as frame_system::Config>::AccountId,
	<T as pallet_balances::Config>::Balance,
	<T as Config>::EraId {
		EraPayout(EraId),
		StakingRewardClaimed(AccountId,Balance),
		/// User stakes funds [staking_account_id, rewards_account_id, staked_balance]
		BalanceStaked(AccountId, AccountId,Balance),
		BalanceUnstaked(AccountId,Balance),
		BalanceReinvested(AccountId,Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		InsufficientFreeBalance,
		InsufficientStakedBalance,
		NoRewardsAvailable,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn stake(origin, rewards_account: T::AccountId, staked_balance: T::Balance) -> dispatch::DispatchResult {
			let user = ensure_signed(origin)?;
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn unstake(origin, unstaked_balance: T::Balance) -> dispatch::DispatchResult {
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