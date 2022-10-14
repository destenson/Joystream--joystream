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

//! Autogenerated weights for storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=storage
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/storage/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for storage.
pub trait WeightInfo {
	fn delete_storage_bucket() -> Weight;
	fn update_uploading_blocked_status() -> Weight;
	fn update_data_size_fee() -> Weight;
	fn update_storage_buckets_per_bag_limit() -> Weight;
	fn update_storage_buckets_voucher_max_limits() -> Weight;
	fn update_data_object_state_bloat_bond() -> Weight;
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight;
	fn update_blacklist(_i: u32, _j: u32, ) -> Weight;
	fn create_storage_bucket() -> Weight;
	fn update_storage_buckets_for_bag(_i: u32, _j: u32, ) -> Weight;
	fn cancel_storage_bucket_operator_invite() -> Weight;
	fn invite_storage_bucket_operator() -> Weight;
	fn remove_storage_bucket_operator() -> Weight;
	fn update_storage_bucket_status() -> Weight;
	fn set_storage_bucket_voucher_limits() -> Weight;
	fn accept_storage_bucket_invitation() -> Weight;
	fn set_storage_operator_metadata(_i: u32, ) -> Weight;
	fn accept_pending_data_objects(_i: u32, ) -> Weight;
	fn create_distribution_bucket_family() -> Weight;
	fn delete_distribution_bucket_family() -> Weight;
	fn create_distribution_bucket() -> Weight;
	fn update_distribution_bucket_status() -> Weight;
	fn delete_distribution_bucket() -> Weight;
	fn update_distribution_buckets_for_bag(_i: u32, _j: u32, ) -> Weight;
	fn update_distribution_buckets_per_bag_limit() -> Weight;
	fn update_distribution_bucket_mode() -> Weight;
	fn update_families_in_dynamic_bag_creation_policy(_i: u32, ) -> Weight;
	fn invite_distribution_bucket_operator() -> Weight;
	fn cancel_distribution_bucket_operator_invite() -> Weight;
	fn remove_distribution_bucket_operator() -> Weight;
	fn set_distribution_bucket_family_metadata(_i: u32, ) -> Weight;
	fn accept_distribution_bucket_invitation() -> Weight;
	fn set_distribution_operator_metadata(_i: u32, ) -> Weight;
	fn storage_operator_remark(_i: u32, ) -> Weight;
	fn distribution_operator_remark(_i: u32, ) -> Weight;
}

/// Weights for storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn delete_storage_bucket() -> Weight {
		(29_030_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage UploadingBlocked (r:0 w:1)
	fn update_uploading_blocked_status() -> Weight {
		(23_890_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DataObjectPerMegabyteFee (r:0 w:1)
	fn update_data_size_fee() -> Weight {
		(24_220_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketsPerBagLimit (r:0 w:1)
	fn update_storage_buckets_per_bag_limit() -> Weight {
		(24_040_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:0 w:1)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:0 w:1)
	fn update_storage_buckets_voucher_max_limits() -> Weight {
		(25_380_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DataObjectStateBloatBondValue (r:0 w:1)
	fn update_data_object_state_bloat_bond() -> Weight {
		(25_151_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:1 w:1)
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight {
		(28_350_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Blacklist (r:1000 w:0)
	// Storage: Storage CurrentBlacklistSize (r:1 w:1)
	fn update_blacklist(i: u32, j: u32, ) -> Weight {
		(936_534_000 as Weight)
			// Standard Error: 399_000
			.saturating_add((9_191_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 399_000
			.saturating_add((1_383_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:1 w:0)
	// Storage: Storage NextStorageBucketId (r:1 w:1)
	// Storage: Storage StorageBucketById (r:0 w:1)
	fn create_storage_bucket() -> Weight {
		(33_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:1)
	// Storage: Storage StorageBucketsPerBagLimit (r:1 w:0)
	// Storage: Storage StorageBucketById (r:14 w:14)
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 3_767_000
			.saturating_add((59_819_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 3_767_000
			.saturating_add((31_645_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn cancel_storage_bucket_operator_invite() -> Weight {
		(59_740_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:2 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn invite_storage_bucket_operator() -> Weight {
		(73_360_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn remove_storage_bucket_operator() -> Weight {
		(60_469_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn update_storage_bucket_status() -> Weight {
		(59_451_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:1 w:0)
	fn set_storage_bucket_voucher_limits() -> Weight {
		(72_280_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn accept_storage_bucket_invitation() -> Weight {
		(55_720_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:0)
	fn set_storage_operator_metadata(_i: u32, ) -> Weight {
		(61_511_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Storage StorageBucketById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:0)
	// Storage: Storage DataObjectsById (r:1 w:1)
	fn accept_pending_data_objects(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 5_000
			.saturating_add((11_648_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyNumber (r:1 w:1)
	// Storage: Storage NextDistributionBucketFamilyId (r:1 w:1)
	// Storage: Storage DistributionBucketFamilyById (r:0 w:1)
	fn create_distribution_bucket_family() -> Weight {
		(31_870_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:1)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:2 w:0)
	// Storage: Storage DistributionBucketFamilyNumber (r:1 w:1)
	fn delete_distribution_bucket_family() -> Weight {
		(46_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:1)
	// Storage: Storage DistributionBucketByFamilyIdById (r:0 w:1)
	fn create_distribution_bucket() -> Weight {
		(33_040_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn update_distribution_bucket_status() -> Weight {
		(33_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn delete_distribution_bucket() -> Weight {
		(32_589_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:1)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:0)
	// Storage: Storage DistributionBucketsPerBagLimit (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:52 w:52)
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(42_524_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((11_974_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 28_000
			.saturating_add((12_331_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketsPerBagLimit (r:0 w:1)
	fn update_distribution_buckets_per_bag_limit() -> Weight {
		(23_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn update_distribution_bucket_mode() -> Weight {
		(33_030_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:2 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:1 w:1)
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight {
		(5_738_000 as Weight)
			// Standard Error: 1_463_000
			.saturating_add((12_923_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:2 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn invite_distribution_bucket_operator() -> Weight {
		(77_011_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn cancel_distribution_bucket_operator_invite() -> Weight {
		(68_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn remove_distribution_bucket_operator() -> Weight {
		(67_960_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:0)
	fn set_distribution_bucket_family_metadata(_i: u32, ) -> Weight {
		(60_332_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn accept_distribution_bucket_invitation() -> Weight {
		(33_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	fn set_distribution_operator_metadata(i: u32, ) -> Weight {
		(34_362_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((32_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:0)
	fn storage_operator_remark(i: u32, ) -> Weight {
		(51_979_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	fn distribution_operator_remark(i: u32, ) -> Weight {
		(56_693_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn delete_storage_bucket() -> Weight {
		0
	}
	fn update_uploading_blocked_status() -> Weight {
		0
	}
	fn update_data_size_fee() -> Weight {
		0
	}
	fn update_storage_buckets_per_bag_limit() -> Weight {
		0
	}
	fn update_storage_buckets_voucher_max_limits() -> Weight {
		0
	}
	fn update_data_object_state_bloat_bond() -> Weight {
		0
	}
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight {
		0
	}
	fn update_blacklist(i: u32, j: u32, ) -> Weight {
		0
	}
	fn create_storage_bucket() -> Weight {
		0
	}
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		0
	}
	fn cancel_storage_bucket_operator_invite() -> Weight {
		0
	}
	fn invite_storage_bucket_operator() -> Weight {
		0
	}
	fn remove_storage_bucket_operator() -> Weight {
		0
	}
	fn update_storage_bucket_status() -> Weight {
		0
	}
	fn set_storage_bucket_voucher_limits() -> Weight {
		0
	}
	fn accept_storage_bucket_invitation() -> Weight {
		0
	}
	fn set_storage_operator_metadata(_i: u32, ) -> Weight {
		0
	}
	fn accept_pending_data_objects(i: u32, ) -> Weight {
		0
	}
	fn create_distribution_bucket_family() -> Weight {
		0
	}
	fn delete_distribution_bucket_family() -> Weight {
		0
	}
	fn create_distribution_bucket() -> Weight {
		0
	}
	fn update_distribution_bucket_status() -> Weight {
		0
	}
	fn delete_distribution_bucket() -> Weight {
		0
	}
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_distribution_buckets_per_bag_limit() -> Weight {
		0
	}
	fn update_distribution_bucket_mode() -> Weight {
		0
	}
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight {
		0
	}
	fn invite_distribution_bucket_operator() -> Weight {
		0
	}
	fn cancel_distribution_bucket_operator_invite() -> Weight {
		0
	}
	fn remove_distribution_bucket_operator() -> Weight {
		0
	}
	fn set_distribution_bucket_family_metadata(_i: u32, ) -> Weight {
		0
	}
	fn accept_distribution_bucket_invitation() -> Weight {
		0
	}
	fn set_distribution_operator_metadata(i: u32, ) -> Weight {
		0
	}
	fn storage_operator_remark(i: u32, ) -> Weight {
		0
	}
	fn distribution_operator_remark(i: u32, ) -> Weight {
		0
	}
}
