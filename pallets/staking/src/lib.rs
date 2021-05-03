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
use pallet_country::Module as Country;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type EraIndex = u32;
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: 
frame_system::Config
+ pallet_balances::Config
+ pallet_country::Config 
+ SendTransactionTypes<Call<Self>> {
	type Event: From<Event<Self>> 
	+ Into<<Self as frame_system::Config>::Event>;
	
	// Constants
	type BalanceLockPeriod: Get<Self::BlockNumber>;
	type EraLength: Get<Self::BlockNumber>;
	type DefaultRewardMultiplier: Get<u32>;

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
		pub EraInformation get(fn era_info): double_map hasher(twox_64_concat) T::BlockNumber, hasher(twox_64_concat) EraIndex => ();
		// Track era 
		pub CurrentEra get(fn current_era): EraIndex;

	}
}

decl_event!(
	pub enum Event<T> where 
	<T as frame_system::Config>::AccountId,
	Balance = BalanceOf<T>,
	 {
		EraPayout(EraIndex),
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
		    Country::<T>::get_country(country).ok_or(Error::<T>::CountryDoesNotExist)?;
			ensure!(T::Currency::free_balance(&account) >= balance, Error::<T>::InsufficientFreeBalance);
			Self::stake_funds(account.clone(),country,balance);
			Self::deposit_event(RawEvent::BalanceStaked(account, country, balance));
		}

		#[weight = 10_000]
		pub fn unstake(origin, country: CountryId, balance: BalanceOf<T>) {
			let account = ensure_signed(origin)?;
			//check if countryId is valid
			let old_balance = Self::staked_balance(&account,country);
			ensure!(balance <=  old_balance, Error::<T>::InsufficientStakedBalance);

			let end_time: T::BlockNumber = <system::Module<T>>::block_number() + T::BalanceLockPeriod::get();
			<UnstakeRequests<T>>::insert(end_time, &account, balance);
	
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
			Self::stake_funds(account.clone(),country,claimed_reward);
			Self::deposit_event(RawEvent::BalanceReinvested(account.clone(), country, claimed_reward));
		}

		fn on_finalize(now: T::BlockNumber) {
			// update era if 
			if now.is_one()  {
				Self::start_new_era();
			}
			if <EraInformation<T>>::contains_key(now, Self::current_era()) {
				Self::calculate_era_rewards_onchain();
				Self::start_new_era();
			}
			//unreserve unstaked funds after end of ther lock period 
			for (account_id, unstaked_funds) in <UnstakeRequests<T>>::drain_prefix(&now) {
				T::Currency::unreserve(&account_id, unstaked_funds);
				Self::deposit_event(RawEvent::UnstakeRequestCompleted(account_id,  unstaked_funds));
			}
			
			
		}
		/*
		#[weight = 10_000]
		pub fn pay_era_rewards(origin, era_reward: BalanceOf<T>, account: T::AccountId, country: CountryId) {
			ensure_none(origin)?;
			<Rewards<T>>::mutate(account, country, |reward| *reward +=  era_reward);
		}
		fn offchain_worker(_now: T::BlockNumber) {
			let _ = Self::calculate_era_rewards_offchain(_now);
		}
		*/
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
	fn stake_funds(account: T::AccountId, country: CountryId, balance: BalanceOf<T>)-> Result<BalanceOf<T>, DispatchError>{
		T::Currency::reserve(&account, balance);
		<StakedBalances<T>>::mutate(&account, country, |staked_balance| *staked_balance += balance);
		Ok(balance)
	}
	fn claim_rewards(account: T::AccountId, country: CountryId) -> Result<BalanceOf<T>, DispatchError> {
		let reward_balance: BalanceOf<T> =  Self::account_rewards(&account,country);
		ensure!(!reward_balance.is_zero(), Error::<T>::NoRewardsAvailable);
		let old_free_balance = T::Currency::free_balance(&account);
		T::Currency::deposit_into_existing(&account, reward_balance).ok();
		<Rewards<T>>::remove(&account,country);
		Ok(reward_balance)
	}
	/*
	fn calculate_era_rewards_offchain(now: T::BlockNumber) -> Result<(), ()> {
		if <EraInformation<T>>::contains_key(now, Self::current_era()) {
			Self::start_new_era();
			//let mut lock = StorageLock::<'_, BlockAndTime<frame_system::Module<T>>>::with_block_deadline(&b"staking/lock"[..], 1);
			//let _guard = lock.try_lock().map_err(|_| ())?;
			for (account_id, country_id, balance) in <StakedBalances<T>>::iter() {
				let mut reward: BalanceOf<T> = balance / T::DefaultRewardMultiplier::get().into();
				let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(Call::<T>::pay_era_rewards(reward,account_id,country_id).into());
			}
			Self::deposit_event(RawEvent::EraPayout(Self::current_era()));
		}
		Ok(())
	}
	*/
	fn calculate_era_rewards_onchain() -> Result<(), ()> {
		for (account_id, country_id, balance) in <StakedBalances<T>>::iter() {
			let mut era_reward: BalanceOf<T> = balance / T::DefaultRewardMultiplier::get().into();
			<Rewards<T>>::mutate(account_id, country_id, |reward| *reward +=  era_reward);
		}
		Self::deposit_event(RawEvent::EraPayout(Self::current_era()));
		Ok(())
	}

	fn start_new_era() -> Result<(), ()>{
		let new_era_index: EraIndex = Self::current_era().saturating_add(1);
		CurrentEra::put(new_era_index);
		if new_era_index.is_one()  {
			<EraInformation<T>>::insert(T::EraLength::get(), new_era_index, ());
		}
		else {
			let new_era_end: T::BlockNumber = <system::Module<T>>::block_number() + T::EraLength::get();
			<EraInformation<T>>::insert(new_era_end, new_era_index, ());
		}
		Ok(())
	}
	
}

impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		match call {
			/*Call::pay_era_rewards(era_reward, account,country) => { 
				ValidTransaction::with_tag_prefix("era_payout")
					.longevity(64_u64)
					.propagate(true)
					.build()
			
			},*/	
			_ => InvalidTransaction::Call.into(),
		}
	}
}
