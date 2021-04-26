#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::string_lit_as_bytes)]

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure,
	traits::{Currency, Get, ReservableCurrency},
	Parameter, IterableStorageDoubleMap,
};

use codec::{Encode, Decode};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One, Zero},
    DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::{self as system, ensure_signed};
use primitives::CountryId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
		// Store all staking rewards for an account
		pub Rewards get(fn get_reward_balance): map hasher(twox_64_concat) T::AccountId => T::Balance;
		// Store all staked balances
		pub StakedBalances get(fn get_staked_balance): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) CountryId => T::Balance;
		// Store all locked balances
		pub UnstakeRequests get(fn get_unstake_request): double_map  hasher(twox_64_concat) T::BlockNumber, hasher(twox_64_concat) T::AccountId
							=> T::Balance;
		// Store era end =
		pub EraEndTime get(fn get_era_info): double_map hasher(twox_64_concat) T::EraId, hasher(twox_64_concat) T::BlockNumber => Option<()>;
		// Track era 
		pub EraIndex get(fn era_index): T::EraId;
	}
}

decl_event!(
	pub enum Event<T> where 
	<T as frame_system::Config>::AccountId,
	<T as pallet_balances::Config>::Balance,
	<T as Config>::EraId {
		EraPayout(EraId),
		StakingRewardClaimed(AccountId, Balance),
		BalanceStaked(AccountId, CountryId, Balance),
		BalanceUnstaked(AccountId, CountryId, Balance),
		UnstakeRequestCompleted(AccountId,Balance),
		BalanceReinvested(AccountId,Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		
		type Error = Error<T>;
		fn deposit_event() = default;

		//#[weight = 10_000 + T::DbWeight::get().writes(1)]
		#[weight = 10_000]
		pub fn stake(origin, country: CountryId, staked_balance: T::Balance) {
			let account = ensure_signed(origin)?;
			ensure!(<pallet_balances::Module<T>>::free_balance(&account) >= staked_balance, Error::<T>::InsufficientFreeBalance);
			<pallet_balances::Module<T>>::reserve(&account, staked_balance);
			<StakedBalances<T>>::insert(&account, country, staked_balance);
			Self::deposit_event(RawEvent::BalanceStaked(account, country, staked_balance));
		}

		//#[weight = 10_000 + T::DbWeight::get().writes(1)]
		#[weight = 10_000]
		pub fn unstake(origin, country: CountryId, unstaked_balance: T::Balance) {
			let account = ensure_signed(origin)?;

			let old_staked_balance = Self::get_staked_balance(&account,country);
			ensure!(unstaked_balance <=  old_staked_balance, Error::<T>::InsufficientStakedBalance);

			let end_time: T::BlockNumber = <system::Module<T>>::block_number() + T::BalanceLockPeriod::get();
			<UnstakeRequests<T>>::insert( end_time, &account, unstaked_balance);
	
			<StakedBalances<T>>::remove(&account,country);
			if old_staked_balance != unstaked_balance {
				<StakedBalances<T>>::insert(&account,country, old_staked_balance - unstaked_balance);
			}
			Self::deposit_event(RawEvent::BalanceUnstaked(account, country, unstaked_balance));
		}

		//#[weight = 10_000 + T::DbWeight::get().writes(1)]
		#[weight = 10_000]
		pub fn claim(origin) {
			let account = ensure_signed(origin)?;
			let reward: T::Balance = Self::claim_rewards(account.clone())?;
			Self::deposit_event(RawEvent::StakingRewardClaimed(account, reward));
		}

		//#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		#[weight = 10_000]
		pub fn reinvest(origin) {
			let account = ensure_signed(origin)?;
			let reward = Self::claim_rewards(account.clone())?;
			<pallet_balances::Module<T>>::reserve(&account, reward);
			Self::deposit_event(RawEvent::BalanceReinvested(account, reward));
		}

		fn on_finalize(now: T::BlockNumber) {
			for (account_id, unstaked_funds) in <UnstakeRequests<T>>::drain_prefix(&now) {
				<pallet_balances::Module<T>>::unreserve(&account_id, unstaked_funds);
				Self::deposit_event(RawEvent::UnstakeRequestCompleted(account_id, unstaked_funds));
			}
		}
    }
}
decl_error! {
	pub enum Error for Module<T: Config> {
		InsufficientFreeBalance,
		InsufficientStakedBalance,
		NoRewardsAvailable,
	}
}
impl<T: Config> Module<T> {
	fn claim_rewards(account: T::AccountId) -> Result<T::Balance, DispatchError> {
		let reward_balance =  Self::get_reward_balance(&account);
		ensure!(!reward_balance.is_zero(), Error::<T>::NoRewardsAvailable);
		let old_free_balance = <pallet_balances::Module<T>>::free_balance(&account);
		let reserved_balance = <pallet_balances::Module<T>>::reserved_balance(&account);
		//<pallet_balances::Module<T>>::set_balance(account, account, old_free_balance + reward_balance, reserved_balance);
		<Rewards<T>>::remove(&account);
		Ok(reward_balance)
	}
}