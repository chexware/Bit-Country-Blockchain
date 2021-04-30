#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure,
	traits::{Currency, Get, ReservableCurrency},
	Parameter, IterableStorageDoubleMap,
};

use codec::{Encode, Decode};
use sp_runtime::{
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
	},
	offchain::storage_lock::{StorageLock, BlockAndTime},
	traits::{AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One, Zero},
    DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::{
		self as system, ensure_signed, ensure_none,
	 	offchain::{SendTransactionTypes, SubmitTransaction}
	};
use primitives::CountryId;
//use pallet_country::Module as Country;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: 
frame_system::Config
+ pallet_balances::Config
//+ pallet_country::Config {
+ SendTransactionTypes<Call<Self>> {
	type Event: From<Event<Self>> 
	+ Into<<Self as frame_system::Config>::Event>;
	
	// Constants
	type BalanceLockPeriod: Get<Self::BlockNumber>;
	type EraLength: Get<Self::BlockNumber>;
	type DefaultRewardMultiplier: Get<u32>;

    /// Type for EraId 
	type EraId: Parameter
    + Member
    + AtLeast32BitUnsigned
    + Default
    + Copy
    + MaybeSerializeDeserialize
    + Bounded;

	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

// The pallet's runtime storage items.
decl_storage! {
	trait Store for Module<T: Config> as StakingModule {
		// Store all staking rewards for an account
		pub Rewards get(fn account_rewards): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) CountryId => BalanceOf<T>;
		// Store all staked balances per co
		pub StakedBalances get(fn staked_balance): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) CountryId => BalanceOf<T>;
		// Store all locked balances
		pub UnstakeRequests get(fn unstake_request): double_map hasher(twox_64_concat) T::BlockNumber, hasher(twox_64_concat) T::AccountId => BalanceOf<T>;
		// Store era end , index, and reward multiplier
		pub EraInformation get(fn era_info): double_map hasher(twox_64_concat) T::BlockNumber, hasher(twox_64_concat) u32 => u32;
		// Track era 
		pub EraIndex get(fn current_era_index): u32;

	}
}

decl_event!(
	pub enum Event<T> where 
	<T as frame_system::Config>::AccountId,
	Balance = BalanceOf<T>,
	 {
		EraPayout(u32),
		StakingRewardClaimed(AccountId, CountryId, Balance),
		BalanceStaked(AccountId, CountryId, Balance),
		BalanceUnstaked(AccountId, CountryId, Balance),
		UnstakeRequestCompleted(AccountId,Balance),
		BalanceReinvested(AccountId, CountryId, Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn stake(origin, country: CountryId, balance: BalanceOf<T>) {
			let account = ensure_signed(origin)?;
			//check if countryId is valid
		//Country::<T>::get_country(country).ok_or(Error::<T>::CountryDoesNotExist)?;
			ensure!(T::Currency::free_balance(&account) >= balance, Error::<T>::InsufficientFreeBalance);
			T::Currency::reserve(&account, balance);
			<StakedBalances<T>>::mutate(&account, country, |staked_balance| *staked_balance += balance);
			Self::deposit_event(RawEvent::BalanceStaked(account, country, balance));
		}

		#[weight = 10_000]
		pub fn unstake(origin, country: CountryId, balance: BalanceOf<T>) {
			let account = ensure_signed(origin)?;
			//check if countryId is valid
			//Country::<T>::get_country(country).ok_or(Error::<T>::CountryDoesNotExist)?;
			let old_balance = Self::staked_balance(&account,country);
			ensure!(balance <=  old_balance, Error::<T>::InsufficientStakedBalance);

			let end_time: T::BlockNumber = <system::Module<T>>::block_number() + T::BalanceLockPeriod::get();
			<UnstakeRequests<T>>::insert( end_time, &account, balance);
	
			<StakedBalances<T>>::remove(&account,country);
			if old_balance != balance {
				<StakedBalances<T>>::insert(&account,country, old_balance - balance);
			}
			Self::deposit_event(RawEvent::BalanceUnstaked(account, country, balance));
		}

		#[weight = 10_000]
		pub fn claim(origin, country: CountryId) {
			let account = ensure_signed(origin)?;
			let reward: BalanceOf<T> = Self::claim_rewards(account.clone(),country)?;
			Self::deposit_event(RawEvent::StakingRewardClaimed(account,country, reward));
		}

		#[weight = 10_000]
		pub fn reinvest(origin, country: CountryId) {
			let account = ensure_signed(origin)?;
			let claimed_reward: BalanceOf<T> = Self::claim_rewards(account.clone(),country)?;
			T::Currency::reserve(&account, claimed_reward);
			<StakedBalances<T>>::mutate(&account, country, |balance| *balance += claimed_reward);
			Self::deposit_event(RawEvent::BalanceReinvested(account.clone(), country, claimed_reward));
		}

		fn on_finalize(now: T::BlockNumber) {
			for (account_id, unstaked_funds) in <UnstakeRequests<T>>::drain_prefix(&now) {
				T::Currency::unreserve(&account_id, unstaked_funds);
				Self::deposit_event(RawEvent::UnstakeRequestCompleted(account_id,  unstaked_funds));
			}
			
			let current_era = Self::current_era_index();
			if (current_era == 0 && now == T::EraLength::get()) || <EraInformation<T>>::contains_key(now, current_era) {
				Self::start_new_era();
				<EraInformation<T>>::remove(now, current_era); 	
			}
		}
		#[weight = 10_000]
		pub fn pay_era_rewards(origin, era_reward: BalanceOf<T>, account: T::AccountId, country: CountryId) {
			ensure_none(origin)?;
			//let old_reward_balance: T::Balance = Self::account_rewards(&account, country);
			<Rewards<T>>::mutate(account, country, |reward| *reward +=  era_reward);
		}
		fn offchain_worker(_now: T::BlockNumber) {
			let _ = Self::calculate_era_rewards();
			Self::deposit_event(RawEvent::EraPayout(Self::current_era_index()));
		}
		
    }
}
decl_error! {
	pub enum Error for Module<T: Config>  {
		CountryDoesNotExist,
		InsufficientFreeBalance,
		InsufficientStakedBalance,
		NoRewardsAvailable,
	}
}
impl<T: Config> Module<T>  {
	fn claim_rewards(account: T::AccountId, country: CountryId) -> Result<BalanceOf<T>, DispatchError> {
		let reward_balance: BalanceOf<T> =  Self::account_rewards(&account,country);
		ensure!(!reward_balance.is_zero(), Error::<T>::NoRewardsAvailable);
		let old_free_balance = T::Currency::free_balance(&account);
		T::Currency::deposit_into_existing(&account, reward_balance).ok();
		<Rewards<T>>::remove(&account,country);
		Ok(reward_balance)
	}

	fn calculate_era_rewards() -> Result<(), ()> {
		let mut lock = StorageLock::<'_, BlockAndTime<frame_system::Module<T>>>::with_block_deadline(&b"staking/lock"[..], 1);
		let _guard = lock.try_lock().map_err(|_| ())?;
		for (account_id, country_id, balance) in <StakedBalances<T>>::iter() {
			let mut reward: BalanceOf<T> = balance / T::DefaultRewardMultiplier::get().into();
			let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(Call::<T>::pay_era_rewards(reward, account_id, country_id).into());
		}
		Ok(())
	}

	fn start_new_era() -> Result<(), ()>{
		let previous_era = Self::current_era_index();
		EraIndex::put(previous_era + 1);
		let new_era_end = <system::Module<T>>::block_number() + T::EraLength::get();
		<EraInformation<T>>::insert(new_era_end, previous_era.saturating_add(1), T::DefaultRewardMultiplier::get());
		Ok(())
	}
	
}

impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		match &*call {
			Call::pay_era_rewards(era_reward, account,country) => {
				ValidTransaction::with_tag_prefix("era_payout")
					.longevity(64_u64)
					.propagate(true)
					.build()
				
			},
			_ => InvalidTransaction::Call.into(),
		}
	}
}