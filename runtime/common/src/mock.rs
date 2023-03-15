use super::*;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, Everything, Nothing},
	weights::Weight,
	PalletId,
};

use frame_system::{EnsureNever, EnsureRoot};
use pallet_evm::{EnsureAddressNever, EnsureAddressRoot};
use precompile_utils::{
	precompile_set::*,
	testing::{MockHandle, PrecompileTesterExt},
};
use sp_core::{U256, H256};
use sp_runtime::traits::{BlakeTwo256, ConstU32, IdentityLookup};

use primitives::{Amount, ClassId, GroupCollectionId, TokenId, FungibleTokenId, ItemId};
use core_primitives::{NftAssetData, NftClassData};
// use auction_manager::{Auction, AuctionInfo, AuctionItem, AuctionType, ListingLevel};
use sp_runtime::Perbill;
use currencies as multi_currency;

pub type AccountId = u128;
pub type AssetId = u128;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
pub type Block = frame_system::mocking::MockBlock<Runtime>;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

/* 
/// The foreign asset precompile address prefix. Addresses that match against this prefix will
/// be routed to Erc20AssetsPrecompileSet being marked as foreign
pub const FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX: u32 = 0xffffffff;

/// The local asset precompile address prefix. Addresses that match against this prefix will
/// be routed to Erc20AssetsPrecompileSet being marked as local
pub const LOCAL_ASSET_PRECOMPILE_ADDRESS_PREFIX: u32 = 0xfffffffe;

parameter_types! {
	pub ForeignAssetPrefix: &'static [u8] = &[0xff, 0xff, 0xff, 0xff];
	pub LocalAssetPrefix: &'static [u8] = &[0xff, 0xff, 0xff, 0xfe];
}

// Implement the trait, where we convert AccountId to AssetID
impl AccountIdAssetIdConversion<AccountId, AssetId> for Runtime {
	/// The way to convert an account to assetId is by ensuring that the prefix is 0XFFFFFFFF
	/// and by taking the lowest 128 bits as the assetId
	fn account_to_asset_id(account: AccountId) -> Option<(Vec<u8>, AssetId)> {
		if account.has_prefix_u32(FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX) {
			return Some((
				FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX
					.to_be_bytes()
					.to_vec(),
				account.without_prefix(),
			));
		}

		if account.has_prefix_u32(LOCAL_ASSET_PRECOMPILE_ADDRESS_PREFIX) {
			return Some((
				LOCAL_ASSET_PRECOMPILE_ADDRESS_PREFIX.to_be_bytes().to_vec(),
				account.without_prefix(),
			));
		}

		None
	}

	// Not used for now
	fn asset_id_to_account(prefix: &[u8], asset_id: AssetId) -> AccountId {
		if prefix
			== LOCAL_ASSET_PRECOMPILE_ADDRESS_PREFIX
				.to_be_bytes()
				.as_slice()
		{
			LocalAssetId(asset_id).into()
		} else {
			ForeignAssetId(asset_id).into()
		}
	}
}
*/
parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 0;
}

impl pallet_balances::Config for Runtime {
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type MaxLocks = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
/* 
pub type Precompiles<R> = PrecompileSetBuilder<
	R,
	(
		PrecompileSetStartingWith<
			ForeignAssetPrefix,
			Erc20AssetsPrecompileSet<R, IsForeign, pallet_assets::Instance1>,
		>,
		PrecompileSetStartingWith<
			LocalAssetPrefix,
			Erc20AssetsPrecompileSet<R, IsLocal, pallet_assets::Instance2>,
		>,
	),
>;
*/

parameter_types! {
	pub BlockGasLimit: U256 = U256::max_value();
	pub PrecompilesValue: Precompiles<Runtime> = Precompiles::new();
	pub WeightPerGas: Weight = Weight::from_ref_time(1);
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type CallOrigin = EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = EnsureAddressNever<AccountId>;
	type AddressMapping = AccountId;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = Precompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ();
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type FindAuthor = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: FungibleTokenId| -> Balance {
		Default::default()
	};
}

parameter_types! {
	pub const MetaverseTreasuryPalletId: PalletId = PalletId(*b"bit/trsy");
	pub TreasuryModuleAccount: AccountId = MetaverseTreasuryPalletId::get().into_account_truncating();
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = FungibleTokenId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryModuleAccount>;
	type MaxLocks = ();
	type ReserveIdentifier = [u8; 8];
	type MaxReserves = ();
	type DustRemovalWhitelist = Nothing;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

pub type AdaptedBasicCurrency = currencies::BasicCurrencyAdapter<Runtime, Balance, Amount, BlockNumber>;

parameter_types! {
	pub const NativeCurrencyId: FungibleTokenId = FungibleTokenId::NativeToken(0);
	pub const MiningCurrencyId: FungibleTokenId = FungibleTokenId::MiningResource(0);
}

impl currencies::Config for Runtime {
	type Event = Event;
	type MultiSocialCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = NativeCurrencyId;
	type WeightInfo = ();
}

/*// NFT - related 
pub struct MockAuctionManager;

impl Auction<AccountId, BlockNumber> for MockAuctionManager {
	type Balance = Balance;

	fn auction_info(_id: u64) -> Option<AuctionInfo<u128, Self::Balance, u64>> {
		None
	}

	fn auction_item(id: AuctionId) -> Option<AuctionItem<AccountId, BlockNumber, Self::Balance>> {
		None
	}

	fn update_auction(_id: u64, _info: AuctionInfo<u128, Self::Balance, u64>) -> DispatchResult {
		Ok(())
	}

	fn update_auction_item(id: AuctionId, item_id: ItemId<Self::Balance>) -> DispatchResult {
		Ok(())
	}

	fn new_auction(
		_recipient: u128,
		_initial_amount: Self::Balance,
		_start: u64,
		_end: Option<u64>,
	) -> Result<u64, DispatchError> {
		Ok(0)
	}

	fn create_auction(
		_auction_type: AuctionType,
		_item_id: ItemId<Balance>,
		_end: Option<u64>,
		_recipient: u128,
		_initial_amount: Self::Balance,
		_start: u64,
		_listing_level: ListingLevel<AccountId>,
		_listing_fee: Perbill,
		_currency_id: FungibleTokenId,
	) -> Result<u64, DispatchError> {
		Ok(0)
	}

	fn remove_auction(_id: u64, _item_id: ItemId<Balance>) {}

	fn auction_bid_handler(from: AccountId, id: AuctionId, value: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn buy_now_handler(from: AccountId, auction_id: AuctionId, value: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn local_auction_bid_handler(
		_: BlockNumber,
		_: u64,
		_: (
			AccountId,
			<Self as auction_manager::Auction<AccountId, BlockNumber>>::Balance,
		),
		_: std::option::Option<(
			AccountId,
			<Self as auction_manager::Auction<AccountId, BlockNumber>>::Balance,
		)>,
		_: FungibleTokenId,
	) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}

	fn collect_royalty_fee(
		_high_bid_price: &Self::Balance,
		_high_bidder: &u128,
		_asset_id: &(u32, u64),
		_social_currency_id: FungibleTokenId,
	) -> DispatchResult {
		Ok(())
	}
}

impl CheckAuctionItemHandler<Balance> for MockAuctionManager {
	fn check_item_in_auction(_item_id: ItemId<Balance>) -> bool {
		return false;
	}
}

parameter_types! {
	pub MaxClassMetadata: u32 = 1024;
	pub MaxTokenMetadata: u32 = 1024;
}

impl orml_nft::Config for Runtime {
	type ClassId = u32;
	type TokenId = u64;
	type Currency = Balances;
	type ClassData = NftClassData<Balance>;
	type TokenData = NftAssetData<Balance>;
	type MaxClassMetadata = MaxClassMetadata;
	type MaxTokenMetadata = MaxTokenMetadata;
}

parameter_types! {
	pub AssetMintingFee: Balance = 1;
	pub ClassMintingFee: Balance = 1;
	pub MaxBatchTransfer: u32 = 100;
	pub MaxBatchMinting: u32 = 1000;
	pub MaxNftMetadata: u32 = 1024;
	pub NftPalletId: PalletId = PalletId(*b"bit/bNFT");
	pub const MetaverseNetworkTreasuryPalletId: PalletId = PalletId(*b"bit/trsy");
}

impl nft::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type Treasury = MetaverseNetworkTreasuryPalletId;
	type WeightInfo = ();
	type PalletId = NftPalletId;
	type AuctionHandler = MockAuctionManager;
	type MaxBatchTransfer = MaxBatchTransfer;
	type MaxBatchMinting = MaxBatchMinting;
	type MaxMetadata = MaxNftMetadata;
	type MiningResourceId = MiningResourceCurrencyId;
	type AssetMintingFee = AssetMintingFee;
	type ClassMintingFee = ClassMintingFee;
}
*/

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Evm: pallet_evm::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
        Currencies: currencies::{ Pallet, Storage, Call, Event<T>}
		//OrmlNft: orml_nft::{Pallet, Storage, Config<T>},
		//Nft: nft::{Pallet, Storage, Call, Event<T>},
		//LocalAssets: pallet_assets::<Instance2>::{Pallet, Call, Storage, Event<T>}
	}
);

pub(crate) struct ExtBuilder {
	// endowed accounts with balances
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> ExtBuilder {
		ExtBuilder { balances: vec![(ALICE, 100000), (BOB, 200000)] }
	}
}

impl ExtBuilder {
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.expect("Frame system builds valid default genesis config");

		pallet_balances::GenesisConfig::<Runtime> {
			balances: self.balances,
		}
		.assimilate_storage(&mut t)
		.expect("Pallet balances storage can be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

