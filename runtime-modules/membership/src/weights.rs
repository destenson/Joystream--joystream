// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-08, STEPS: `3`, REPEAT: 3, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=membership
// --extrinsic=*
// --chain=dev
// --steps=3
// --repeat=3
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for membership.
pub trait WeightInfo {
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight;
	fn buy_membership_with_referrer(i: u32, j: u32, ) -> Weight;
	fn update_profile(i: u32, ) -> Weight;
	fn update_accounts_none() -> Weight;
	fn update_accounts_root() -> Weight;
	fn update_accounts_controller() -> Weight;
	fn update_accounts_both() -> Weight;
	fn set_referral_cut() -> Weight;
	fn transfer_invites() -> Weight;
	fn invite_member(i: u32, j: u32, ) -> Weight;
	fn gift_membership(i: u32, j: u32, ) -> Weight;
	fn set_membership_price() -> Weight;
	fn update_profile_verification() -> Weight;
	fn set_leader_invitation_quota() -> Weight;
	fn set_initial_invitation_balance() -> Weight;
	fn set_initial_invitation_count() -> Weight;
	fn add_staking_account_candidate() -> Weight;
	fn confirm_staking_account() -> Weight;
	fn remove_staking_account() -> Weight;
	fn member_remark() -> Weight;
}

/// Weights for membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc445c60ee96be8ba74dcbcf1be5859b72] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc6b8a2f06065bb1b4d3395eb9f6a6ce60] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc0d3094b474c99662ab2c5f2e2f8c27b6] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc4560c4f4db6b0f2ccd9ba01ebd9f4757] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:0 w:1)
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		(29_016_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc445c60ee96be8ba74dcbcf1be5859b72] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc6b8a2f06065bb1b4d3395eb9f6a6ce60] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc0d3094b474c99662ab2c5f2e2f8c27b6] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc4560c4f4db6b0f2ccd9ba01ebd9f4757] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bccaf1c595cf48e2f63ea3f59e0a8ea0f4] (r:1 w:0)
	fn buy_membership_with_referrer(i: u32, _j: u32, ) -> Weight {
		(45_799_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc6b8a2f06065bb1b4d3395eb9f6a6ce60] (r:1 w:2)
	fn update_profile(i: u32, ) -> Weight {
		(20_767_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn update_accounts_none() -> Weight {
		(0 as Weight)
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	fn update_accounts_root() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	fn update_accounts_controller() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	fn update_accounts_both() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bccaf1c595cf48e2f63ea3f59e0a8ea0f4] (r:0 w:1)
	fn set_referral_cut() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:2)
	fn transfer_invites() -> Weight {
		(18_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:2)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc6b8a2f06065bb1b4d3395eb9f6a6ce60] (r:1 w:1)
	// Storage: unknown [0x30f7f927af3a1cd6f254a51fb5ddb9e7f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc63049340d8ae4aa11defda0342865772] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc4560c4f4db6b0f2ccd9ba01ebd9f4757] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn invite_member(i: u32, j: u32, ) -> Weight {
		(41_348_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc6b8a2f06065bb1b4d3395eb9f6a6ce60] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc445c60ee96be8ba74dcbcf1be5859b72] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:3 w:3)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:2 w:2)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc4560c4f4db6b0f2ccd9ba01ebd9f4757] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:0 w:1)
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		(81_081_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc445c60ee96be8ba74dcbcf1be5859b72] (r:0 w:1)
	fn set_membership_price() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x30f7f927af3a1cd6f254a51fb5ddb9e7b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	fn update_profile_verification() -> Weight {
		(18_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x30f7f927af3a1cd6f254a51fb5ddb9e73091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x30f7f927af3a1cd6f254a51fb5ddb9e7b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:1)
	fn set_leader_invitation_quota() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc63049340d8ae4aa11defda0342865772] (r:0 w:1)
	fn set_initial_invitation_balance() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc0d3094b474c99662ab2c5f2e2f8c27b6] (r:0 w:1)
	fn set_initial_invitation_count() -> Weight {
		(8_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc8b18453086fa74ddec96f7b48109d8f4] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn add_staking_account_candidate() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc8b18453086fa74ddec96f7b48109d8f4] (r:1 w:1)
	fn confirm_staking_account() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc8b18453086fa74ddec96f7b48109d8f4] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn remove_staking_account() -> Weight {
		(30_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn member_remark() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		0
	}
	fn buy_membership_with_referrer(i: u32, _j: u32, ) -> Weight {
		0
	}
	fn update_profile(i: u32, ) -> Weight {
		0
	}
	fn update_accounts_none() -> Weight {
		0
	}
	fn update_accounts_root() -> Weight {
		0
	}
	fn update_accounts_controller() -> Weight {
		0
	}
	fn update_accounts_both() -> Weight {
		0
	}
	fn set_referral_cut() -> Weight {
		0
	}
	fn transfer_invites() -> Weight {
		0
	}
	fn invite_member(_i: u32, _j: u32, ) -> Weight {
		0
	}
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		0
	}
	fn set_membership_price() -> Weight {
		0
	}
	fn update_profile_verification() -> Weight {
		0
	}
	fn set_leader_invitation_quota() -> Weight {
		0
	}
	fn set_initial_invitation_balance() -> Weight {
		0
	}
	fn set_initial_invitation_count() -> Weight {
		0
	}
	fn add_staking_account_candidate() -> Weight {
		0
	}
	fn confirm_staking_account() -> Weight {
		0
	}
	fn remove_staking_account() -> Weight {
		0
	}
	fn member_remark() -> Weight {
		0
	}
}
