#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	traits::{Currency, Get, ReservableCurrency},
	Parameter, IterableStorageDoubleMap,
};

use codec::{Encode, Decode};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One, Zero},
    DispatchError, DispatchResult, RuntimeDebug,
};
use frame_system::{self as system, ensure_signed };
use primitives::{CountryId};
use pallet_country::Module as Country;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::OriginFor;
    use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	pub(super) type EraIndex = u32;
	pub(super) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config
	+ pallet_balances::Config
	+ pallet_country::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		#[pallet::constant]
		type BalanceLockPeriod: Get<Self::BlockNumber>;
		#[pallet::constant]
		type EraLength: Get<Self::BlockNumber>;
		#[pallet::constant]
		type DefaultRewardMultiplier: Get<u32>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	
	#[pallet::storage]
	#[pallet::getter(fn account_rewards)] 
	pub(super) type Rewards<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, CountryId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn staked_balance)] 
	pub(super) type StakedBalances<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, CountryId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn unstake_request)] 		
	pub(super) type UnstakeRequests<T: Config> = StorageDoubleMap<_, Twox64Concat, T::BlockNumber, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn era_info)] 
	pub(super) type EraInformation<T: Config> = StorageDoubleMap<_, Twox64Concat, T::BlockNumber, Twox64Concat, EraIndex,(), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn current_era)] 
	pub(super) type CurrentEra<T: Config> =  StorageValue<_,EraIndex, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata()]
	pub enum Event<T: Config> {
		EraPayout(EraIndex),
		StakingRewardClaimed(T::AccountId, CountryId, BalanceOf<T>),
		BalanceStaked(T::AccountId, CountryId, BalanceOf<T>),
		BalanceUnstaked(T::AccountId, CountryId, BalanceOf<T>),
		UnstakeRequestCompleted(T::AccountId,BalanceOf<T>),
		BalanceReinvested(T::AccountId, CountryId, BalanceOf<T>),
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {	
		
		#[pallet::weight(10_000)]
		pub(super) fn stake(origin: OriginFor<T>, country: CountryId, balance: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			//check if countryId is valid
			Country::<T>::get_country(country).ok_or(Error::<T>::CountryDoesNotExist)?;
			ensure!(T::Currency::free_balance(&account) >= balance, Error::<T>::InsufficientFreeBalance);
			Self::stake_funds(account.clone(),country,balance);
			Self::deposit_event(Event::<T>::BalanceStaked(account, country, balance));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub(super) fn unstake(origin: OriginFor<T>, country: CountryId, balance: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			//check if countryId is valid
			let old_balance = Self::staked_balance(&account,country);
			ensure!(balance <=  old_balance, Error::<T>::InsufficientStakedBalance);

			let end_time: T::BlockNumber = <system::Module<T>>::block_number() + T::BalanceLockPeriod::get();
			UnstakeRequests::<T>::insert(end_time, &account, balance);

			StakedBalances::<T>::remove(&account,country);
			if old_balance != balance {
				StakedBalances::<T>::insert(&account,country, old_balance - balance);
			}
			Self::deposit_event(Event::<T>::BalanceUnstaked(account, country, balance));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub(super) fn claim(origin: OriginFor<T>, country: CountryId) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let reward: BalanceOf<T> = Self::claim_rewards(account.clone(),country)?;
			Self::deposit_event(Event::<T>::StakingRewardClaimed(account,country, reward));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub(super) fn reinvest(origin: OriginFor<T>, country: CountryId)-> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let claimed_reward: BalanceOf<T> = Self::claim_rewards(account.clone(),country)?;
			Self::stake_funds(account.clone(),country,claimed_reward);
			Self::deposit_event(Event::<T>::BalanceReinvested(account.clone(), country, claimed_reward));
			Ok(().into())
		}

		
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(now: T::BlockNumber) {
			// update era 
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
				Self::deposit_event(Event::<T>::UnstakeRequestCompleted(account_id,  unstaked_funds));
			}
			
			
		}
	}


	#[pallet::error]
	pub enum Error<T>  {
		CountryDoesNotExist,
		InsufficientFreeBalance,
		InsufficientStakedBalance,
		NoRewardsAvailable,
	}

	impl<T: Config> Pallet<T>  {
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

		fn calculate_era_rewards_onchain() -> Result<(), ()> {
			for (account_id, country_id, balance) in <StakedBalances<T>>::iter() {
				let mut era_reward: BalanceOf<T> = balance / T::DefaultRewardMultiplier::get().into();
				<Rewards<T>>::mutate(account_id, country_id, |reward| *reward +=  era_reward);
			}
			Self::deposit_event(Event::<T>::EraPayout(Self::current_era()));
			Ok(())
		}

		fn start_new_era() -> Result<(), ()>{
			let new_era_index: EraIndex = Self::current_era().saturating_add(1);
			<CurrentEra<T>>::put(new_era_index);
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
}
