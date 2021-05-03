#![cfg(test)]

use super::*;

use crate as staking_module;
use frame_support::{
    construct_runtime, impl_outer_event, impl_outer_origin, impl_outer_dispatch, parameter_types,
    traits::{OnInitialize, OnFinalize, EnsureOrigin},
};
use sp_core::H256;
use sp_runtime::{
    testing::{Header,TestXt}, traits::IdentityLookup, 
    ModuleId, Perbill, 
};
use primitives::{CurrencyId, Amount, BlockNumber,CountryId};


parameter_types! {
	pub const BlockHashCount: u32 = 256;
}

pub type AccountId = u128;
pub type Balance = u64;

pub const ALICE: AccountId = 3;
pub const BOB: AccountId = 9;
pub const COUNTRY: CountryId = 0;
pub const COUNTRY_ID_NOT_EXIST: CountryId = 2;

impl system::Config for Runtime {
	type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}


pub type Extrinsic = TestXt<Call, ()>;

parameter_types! {
    pub const BalanceLockPeriod: u32 = 100; //Test lock period is 100 blocks
	pub const EraLength: u32 = 200; //Test era length is 200 blocks
    pub const DefaultRewardMultiplier: u32 = 10;
}

impl Config for Runtime {
    type Event = Event;
	type BalanceLockPeriod = BalanceLockPeriod;
    type DefaultRewardMultiplier = DefaultRewardMultiplier;
	type EraLength: = EraLength;
    type Currency = Balances;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type WeightInfo = ();
}

parameter_types! {
	pub const CountryFundModuleId: ModuleId = ModuleId(*b"bit/fund");
}

impl pallet_country::Config for Runtime {
	type Event = Event;
	type ModuleId = CountryFundModuleId;
}


use frame_system::Call as SystemCall;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        CountryModule: pallet_country::{Module, Call, Storage,Event<T>},
        StakingModule: staking_module::{Module, Call, Storage, Event<T>},
	}
);
pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: vec![(ALICE, 100000),(BOB,2000)],
        }
            .assimilate_storage(&mut t)
            .unwrap();
        
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn last_event() -> Event {
    frame_system::Module::<Runtime>::events()
        .pop()
        .expect("Event expected")
        .event
}


// Simulate block production
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		
		StakingModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		StakingModule::on_initialize(System::block_number());
	}
}


