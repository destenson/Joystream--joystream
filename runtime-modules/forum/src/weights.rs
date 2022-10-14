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

//! Autogenerated weights for forum
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=forum
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/forum/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for forum.
pub trait WeightInfo {
	fn create_category(_i: u32, _j: u32, _k: u32, ) -> Weight;
	fn update_category_membership_of_moderator_new() -> Weight;
	fn update_category_membership_of_moderator_old() -> Weight;
	fn update_category_archival_status_lead(_i: u32, ) -> Weight;
	fn update_category_archival_status_moderator(_i: u32, ) -> Weight;
	fn update_category_title_lead(_i: u32, _j: u32, ) -> Weight;
	fn update_category_title_moderator(_i: u32, _j: u32, ) -> Weight;
	fn update_category_description_lead(_i: u32, _j: u32, ) -> Weight;
	fn update_category_description_moderator(_i: u32, _j: u32, ) -> Weight;
	fn delete_category_lead(_i: u32, ) -> Weight;
	fn delete_category_moderator(_i: u32, ) -> Weight;
	fn create_thread(_i: u32, _j: u32, _k: u32, ) -> Weight;
	fn edit_thread_metadata(_i: u32, _j: u32, ) -> Weight;
	fn delete_thread(_i: u32, ) -> Weight;
	fn move_thread_to_category_lead(_i: u32, ) -> Weight;
	fn move_thread_to_category_moderator(_i: u32, ) -> Weight;
	fn moderate_thread_lead(_i: u32, _k: u32, ) -> Weight;
	fn moderate_thread_moderator(_i: u32, _k: u32, ) -> Weight;
	fn add_post(_i: u32, _j: u32, ) -> Weight;
	fn edit_post_text(_i: u32, _j: u32, ) -> Weight;
	fn moderate_post_lead(_i: u32, _j: u32, ) -> Weight;
	fn moderate_post_moderator(_i: u32, _j: u32, ) -> Weight;
	fn delete_posts(_i: u32, _j: u32, _k: u32, ) -> Weight;
	fn set_stickied_threads_lead(_i: u32, _j: u32, ) -> Weight;
	fn set_stickied_threads_moderator(_i: u32, _j: u32, ) -> Weight;
}

/// Weights for forum using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryCounter (r:1 w:1)
	// Storage: Forum_1_1 NextCategoryId (r:1 w:1)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	fn create_category(i: u32, j: u32, k: u32, ) -> Weight {
		(30_349_000 as Weight)
			// Standard Error: 76_000
			.saturating_add((6_098_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:0 w:1)
	fn update_category_membership_of_moderator_new() -> Weight {
		(39_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:1)
	fn update_category_membership_of_moderator_old() -> Weight {
		(41_650_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	fn update_category_archival_status_lead(i: u32, ) -> Weight {
		(28_075_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((4_259_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	fn update_category_archival_status_moderator(i: u32, ) -> Weight {
		(30_051_000 as Weight)
			// Standard Error: 75_000
			.saturating_add((4_276_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	fn update_category_title_lead(i: u32, j: u32, ) -> Weight {
		(30_401_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((4_299_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	fn update_category_title_moderator(i: u32, j: u32, ) -> Weight {
		(31_148_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((4_469_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	fn update_category_description_lead(i: u32, j: u32, ) -> Weight {
		(29_248_000 as Weight)
			// Standard Error: 41_000
			.saturating_add((4_491_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	fn update_category_description_moderator(i: u32, j: u32, ) -> Weight {
		(31_688_000 as Weight)
			// Standard Error: 34_000
			.saturating_add((4_393_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryCounter (r:1 w:1)
	fn delete_category_lead(i: u32, ) -> Weight {
		(28_768_000 as Weight)
			// Standard Error: 92_000
			.saturating_add((5_275_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:2 w:2)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	// Storage: Forum_1_1 CategoryCounter (r:1 w:1)
	fn delete_category_moderator(i: u32, ) -> Weight {
		(33_839_000 as Weight)
			// Standard Error: 109_000
			.saturating_add((4_952_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Forum_1_1 NextThreadId (r:1 w:1)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 NextPostId (r:1 w:1)
	// Storage: Forum_1_1 PostById (r:0 w:1)
	fn create_thread(i: u32, j: u32, k: u32, ) -> Weight {
		(108_254_000 as Weight)
			// Standard Error: 80_000
			.saturating_add((4_964_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	fn edit_thread_metadata(i: u32, j: u32, ) -> Weight {
		(28_846_000 as Weight)
			// Standard Error: 42_000
			.saturating_add((4_580_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn delete_thread(i: u32, ) -> Weight {
		(64_738_000 as Weight)
			// Standard Error: 93_000
			.saturating_add((4_252_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:2 w:2)
	// Storage: Forum_1_1 ThreadById (r:1 w:2)
	fn move_thread_to_category_lead(i: u32, ) -> Weight {
		(48_529_000 as Weight)
			// Standard Error: 140_000
			.saturating_add((4_996_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:2 w:2)
	// Storage: Forum_1_1 CategoryByModerator (r:2 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:2)
	fn move_thread_to_category_moderator(i: u32, ) -> Weight {
		(56_230_000 as Weight)
			// Standard Error: 164_000
			.saturating_add((5_072_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn moderate_thread_lead(i: u32, k: u32, ) -> Weight {
		(65_557_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((4_355_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn moderate_thread_moderator(i: u32, k: u32, ) -> Weight {
		(67_121_000 as Weight)
			// Standard Error: 59_000
			.saturating_add((4_372_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Forum_1_1 NextPostId (r:1 w:1)
	// Storage: Forum_1_1 PostById (r:0 w:1)
	fn add_post(i: u32, j: u32, ) -> Weight {
		(86_653_000 as Weight)
			// Standard Error: 68_000
			.saturating_add((4_752_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:0)
	// Storage: Forum_1_1 PostById (r:1 w:1)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	fn edit_post_text(i: u32, j: u32, ) -> Weight {
		(44_018_000 as Weight)
			// Standard Error: 53_000
			.saturating_add((4_285_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 PostById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn moderate_post_lead(i: u32, j: u32, ) -> Weight {
		(76_822_000 as Weight)
			// Standard Error: 63_000
			.saturating_add((5_729_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 PostById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn moderate_post_moderator(i: u32, j: u32, ) -> Weight {
		(77_402_000 as Weight)
			// Standard Error: 59_000
			.saturating_add((5_998_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Forum_1_1 ThreadById (r:1 w:1)
	// Storage: Forum_1_1 PostById (r:500 w:500)
	// Storage: Forum_1_1 CategoryById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn delete_posts(i: u32, _j: u32, k: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 3_300_000
			.saturating_add((802_356_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 20_000
			.saturating_add((52_574_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 ThreadById (r:20 w:0)
	fn set_stickied_threads_lead(i: u32, j: u32, ) -> Weight {
		(30_186_000 as Weight)
			// Standard Error: 77_000
			.saturating_add((4_343_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 18_000
			.saturating_add((7_280_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1WorkingGroup WorkerById (r:1 w:0)
	// Storage: Forum_1_1 CategoryById (r:1 w:1)
	// Storage: Forum_1_1 CategoryByModerator (r:1 w:0)
	// Storage: Forum_1_1 ThreadById (r:20 w:0)
	fn set_stickied_threads_moderator(i: u32, j: u32, ) -> Weight {
		(27_548_000 as Weight)
			// Standard Error: 82_000
			.saturating_add((4_858_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 19_000
			.saturating_add((7_613_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_category(i: u32, j: u32, k: u32, ) -> Weight {
		0
	}
	fn update_category_membership_of_moderator_new() -> Weight {
		0
	}
	fn update_category_membership_of_moderator_old() -> Weight {
		0
	}
	fn update_category_archival_status_lead(i: u32, ) -> Weight {
		0
	}
	fn update_category_archival_status_moderator(i: u32, ) -> Weight {
		0
	}
	fn update_category_title_lead(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_category_title_moderator(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_category_description_lead(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_category_description_moderator(i: u32, j: u32, ) -> Weight {
		0
	}
	fn delete_category_lead(i: u32, ) -> Weight {
		0
	}
	fn delete_category_moderator(i: u32, ) -> Weight {
		0
	}
	fn create_thread(i: u32, j: u32, k: u32, ) -> Weight {
		0
	}
	fn edit_thread_metadata(i: u32, j: u32, ) -> Weight {
		0
	}
	fn delete_thread(i: u32, ) -> Weight {
		0
	}
	fn move_thread_to_category_lead(i: u32, ) -> Weight {
		0
	}
	fn move_thread_to_category_moderator(i: u32, ) -> Weight {
		0
	}
	fn moderate_thread_lead(i: u32, k: u32, ) -> Weight {
		0
	}
	fn moderate_thread_moderator(i: u32, k: u32, ) -> Weight {
		0
	}
	fn add_post(i: u32, j: u32, ) -> Weight {
		0
	}
	fn edit_post_text(i: u32, j: u32, ) -> Weight {
		0
	}
	fn moderate_post_lead(i: u32, j: u32, ) -> Weight {
		0
	}
	fn moderate_post_moderator(i: u32, j: u32, ) -> Weight {
		0
	}
	fn delete_posts(i: u32, _j: u32, k: u32, ) -> Weight {
		0
	}
	fn set_stickied_threads_lead(i: u32, j: u32, ) -> Weight {
		0
	}
	fn set_stickied_threads_moderator(i: u32, j: u32, ) -> Weight {
		0
	}
}
