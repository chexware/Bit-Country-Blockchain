// This file is part of Metaverse.Network & Bit.Country.

// Copyright (C) 2020-2022 Metaverse.Network & Bit.Country .
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-21, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/metaverse-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// nft
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --template=./template/weight-template.hbs
// --output
// ./pallets/nft/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for nft.
pub trait WeightInfo {	fn create_group() -> Weight;	fn create_class() -> Weight;	fn mint() -> Weight;	fn mint_stackable_nft() -> Weight;	fn transfer() -> Weight;	fn transfer_stackable_nft() -> Weight;	fn transfer_batch() -> Weight;	fn sign_asset() -> Weight;	fn set_hard_limit() -> Weight;	fn withdraw_funds_from_class_fund() -> Weight;	fn force_update_total_issuance() -> Weight;}

/// Weights for nft using the for collator node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	// Storage: Nft NextGroupCollectionId (r:1 w:1)
	// Proof Skipped: Nft NextGroupCollectionId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Nft AllNftGroupCollection (r:1 w:1)
	// Proof Skipped: Nft AllNftGroupCollection (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Nft GroupCollections (r:0 w:1)
	// Proof Skipped: Nft GroupCollections (max_values: None, max_size: None, mode: Measured)
	fn create_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1317`
		// Minimum execution time: 11_400 nanoseconds.
		Weight::from_parts(12_179_000, 1317)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: OrmlNFT NextClassId (r:1 w:1)
	// Proof Skipped: OrmlNFT NextClassId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Nft GroupCollections (r:1 w:0)
	// Proof Skipped: Nft GroupCollections (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: OrmlNFT Classes (r:0 w:1)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: Nft ClassDataCollection (r:0 w:1)
	// Proof Skipped: Nft ClassDataCollection (max_values: None, max_size: None, mode: Measured)
	fn create_class() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461`
		//  Estimated: `7417`
		// Minimum execution time: 26_854 nanoseconds.
		Weight::from_parts(27_797_000, 7417)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Nft LockedCollection (r:1 w:0)
	// Proof Skipped: Nft LockedCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: OrmlNFT NextTokenId (r:1 w:1)
	// Proof Skipped: OrmlNFT NextTokenId (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Tokens (r:3 w:3)
	// Proof Skipped: OrmlNFT Tokens (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT TokensByOwner (r:0 w:3)
	// Proof Skipped: OrmlNFT TokensByOwner (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `784`
		//  Estimated: `23976`
		// Minimum execution time: 52_673 nanoseconds.
		Weight::from_parts(54_185_000, 23976)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: OrmlNFT NextTokenId (r:1 w:1)
	// Proof Skipped: OrmlNFT NextTokenId (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT StackableCollectionsBalances (r:1 w:1)
	// Proof Skipped: OrmlNFT StackableCollectionsBalances (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT StackableCollection (r:0 w:1)
	// Proof Skipped: OrmlNFT StackableCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Tokens (r:0 w:1)
	// Proof Skipped: OrmlNFT Tokens (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	// Proof Skipped: OrmlNFT TokensByOwner (max_values: None, max_size: None, mode: Measured)
	fn mint_stackable_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `550`
		//  Estimated: `15931`
		// Minimum execution time: 39_871 nanoseconds.
		Weight::from_parts(41_846_000, 15931)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Auction ItemsInAuction (r:1 w:0)
	// Proof Skipped: Auction ItemsInAuction (max_values: None, max_size: None, mode: Measured)
	// Storage: Nft LockedCollection (r:1 w:0)
	// Proof Skipped: Nft LockedCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT StackableCollection (r:1 w:0)
	// Proof Skipped: OrmlNFT StackableCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Proof Skipped: OrmlNFT Tokens (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT TokensByOwner (r:0 w:2)
	// Proof Skipped: OrmlNFT TokensByOwner (max_values: None, max_size: None, mode: Measured)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `802`
		//  Estimated: `17187`
		// Minimum execution time: 30_774 nanoseconds.
		Weight::from_parts(32_122_000, 17187)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Auction ItemsInAuction (r:1 w:0)
	// Proof Skipped: Auction ItemsInAuction (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT StackableCollectionsBalances (r:2 w:2)
	// Proof Skipped: OrmlNFT StackableCollectionsBalances (max_values: None, max_size: None, mode: Measured)
	// Storage: Nft ReservedStackableNftBalance (r:1 w:0)
	// Proof Skipped: Nft ReservedStackableNftBalance (max_values: None, max_size: None, mode: Measured)
	fn transfer_stackable_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `630`
		//  Estimated: `11790`
		// Minimum execution time: 26_888 nanoseconds.
		Weight::from_parts(28_373_000, 11790)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Auction ItemsInAuction (r:2 w:0)
	// Proof Skipped: Auction ItemsInAuction (max_values: None, max_size: None, mode: Measured)
	// Storage: Nft LockedCollection (r:1 w:0)
	// Proof Skipped: Nft LockedCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT StackableCollection (r:2 w:0)
	// Proof Skipped: OrmlNFT StackableCollection (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT Tokens (r:2 w:2)
	// Proof Skipped: OrmlNFT Tokens (max_values: None, max_size: None, mode: Measured)
	// Storage: OrmlNFT TokensByOwner (r:0 w:4)
	// Proof Skipped: OrmlNFT TokensByOwner (max_values: None, max_size: None, mode: Measured)
	fn transfer_batch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `980`
		//  Estimated: `25680`
		// Minimum execution time: 109_148 nanoseconds.
		Weight::from_parts(112_127_000, 25680)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Proof Skipped: OrmlNFT Tokens (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Nft AssetSupporters (r:1 w:1)
	// Proof Skipped: Nft AssetSupporters (max_values: None, max_size: None, mode: Measured)
	fn sign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `867`
		//  Estimated: `11890`
		// Minimum execution time: 35_431 nanoseconds.
		Weight::from_parts(38_782_000, 11890)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	fn set_hard_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `2757`
		// Minimum execution time: 11_687 nanoseconds.
		Weight::from_parts(15_065_000, 2757)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_funds_from_class_fund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `587`
		//  Estimated: `8268`
		// Minimum execution time: 24_825 nanoseconds.
		Weight::from_parts(25_576_000, 8268)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	fn force_update_total_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `2757`
		// Minimum execution time: 10_502 nanoseconds.
		Weight::from_parts(11_211_000, 2757)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {	fn create_group() -> Weight {
		Weight::from_parts(12_179_000, 1317)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	fn create_class() -> Weight {
		Weight::from_parts(27_797_000, 7417)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	fn mint() -> Weight {
		Weight::from_parts(54_185_000, 23976)
			.saturating_add(RocksDbWeight::get().reads(8))
			.saturating_add(RocksDbWeight::get().writes(10))
	}
	fn mint_stackable_nft() -> Weight {
		Weight::from_parts(41_846_000, 15931)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
	fn transfer() -> Weight {
		Weight::from_parts(32_122_000, 17187)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	fn transfer_stackable_nft() -> Weight {
		Weight::from_parts(28_373_000, 11790)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	fn transfer_batch() -> Weight {
		Weight::from_parts(112_127_000, 25680)
			.saturating_add(RocksDbWeight::get().reads(8))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	fn sign_asset() -> Weight {
		Weight::from_parts(38_782_000, 11890)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	fn set_hard_limit() -> Weight {
		Weight::from_parts(15_065_000, 2757)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn withdraw_funds_from_class_fund() -> Weight {
		Weight::from_parts(25_576_000, 8268)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	fn force_update_total_issuance() -> Weight {
		Weight::from_parts(11_211_000, 2757)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
