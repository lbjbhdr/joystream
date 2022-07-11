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
//! DATE: 2022-07-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
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
	fn update_blacklist(i: u32, j: u32, ) -> Weight;
	fn create_storage_bucket() -> Weight;
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight;
	fn cancel_storage_bucket_operator_invite() -> Weight;
	fn invite_storage_bucket_operator() -> Weight;
	fn remove_storage_bucket_operator() -> Weight;
	fn update_storage_bucket_status() -> Weight;
	fn set_storage_bucket_voucher_limits() -> Weight;
	fn accept_storage_bucket_invitation() -> Weight;
	fn set_storage_operator_metadata(i: u32, ) -> Weight;
	fn accept_pending_data_objects(i: u32, ) -> Weight;
	fn create_distribution_bucket_family() -> Weight;
	fn delete_distribution_bucket_family() -> Weight;
	fn create_distribution_bucket() -> Weight;
	fn update_distribution_bucket_status() -> Weight;
	fn delete_distribution_bucket() -> Weight;
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight;
	fn update_distribution_buckets_per_bag_limit() -> Weight;
	fn update_distribution_bucket_mode() -> Weight;
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight;
	fn invite_distribution_bucket_operator() -> Weight;
	fn cancel_distribution_bucket_operator_invite() -> Weight;
	fn remove_distribution_bucket_operator() -> Weight;
	fn set_distribution_bucket_family_metadata(i: u32, ) -> Weight;
	fn accept_distribution_bucket_invitation() -> Weight;
	fn set_distribution_operator_metadata(i: u32, ) -> Weight;
	fn storage_operator_remark(i: u32, ) -> Weight;
	fn distribution_operator_remark(i: u32, ) -> Weight;
}

/// Weights for storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn delete_storage_bucket() -> Weight {
		(31_106_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:0 w:1)
	fn update_uploading_blocked_status() -> Weight {
		(26_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:0 w:1)
	fn update_data_size_fee() -> Weight {
		(26_766_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adca76b2864d8d9db3061f358145999566] (r:0 w:1)
	fn update_storage_buckets_per_bag_limit() -> Weight {
		(26_697_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad2111f497c75576d1d7fe04ad50625504] (r:0 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad5843345615087f0d2adfd31353080cef] (r:0 w:1)
	fn update_storage_buckets_voucher_max_limits() -> Weight {
		(27_616_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:0 w:1)
	fn update_data_object_state_bloat_bond() -> Weight {
		(26_566_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:1 w:1)
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight {
		(31_533_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:10000 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7bba30747cc66cc066042a6ee636a08] (r:1 w:1)
	fn update_blacklist(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((4_769_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 14_000
			.saturating_add((2_917_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad2111f497c75576d1d7fe04ad50625504] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad5843345615087f0d2adfd31353080cef] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad71a246a8c38edeb0916d527ed249b28b] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:0 w:1)
	fn create_storage_bucket() -> Weight {
		(35_702_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adca76b2864d8d9db3061f358145999566] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:21 w:21)
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(30_432_000 as Weight)
			// Standard Error: 94_000
			.saturating_add((17_774_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 94_000
			.saturating_add((15_482_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn cancel_storage_bucket_operator_invite() -> Weight {
		(36_046_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:2 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn invite_storage_bucket_operator() -> Weight {
		(41_607_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn remove_storage_bucket_operator() -> Weight {
		(36_590_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn update_storage_bucket_status() -> Weight {
		(34_311_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d23091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad2111f497c75576d1d7fe04ad50625504] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad5843345615087f0d2adfd31353080cef] (r:1 w:0)
	fn set_storage_bucket_voucher_limits() -> Weight {
		(39_536_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:1)
	fn accept_storage_bucket_invitation() -> Weight {
		(33_404_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:0)
	fn set_storage_operator_metadata(_i: u32, ) -> Weight {
		(31_833_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:1 w:1)
	fn accept_pending_data_objects(i: u32, ) -> Weight {
		(15_017_000 as Weight)
			// Standard Error: 26_000
			.saturating_add((10_496_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adba42281af3f2c6313d2ab9c6b4cef099] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad4e3d469977d9f07154b2777874deeab1] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:0 w:1)
	fn create_distribution_bucket_family() -> Weight {
		(33_883_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:2 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adba42281af3f2c6313d2ab9c6b4cef099] (r:1 w:1)
	fn delete_distribution_bucket_family() -> Weight {
		(48_904_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:0 w:1)
	fn create_distribution_bucket() -> Weight {
		(36_002_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn update_distribution_bucket_status() -> Weight {
		(37_307_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn delete_distribution_bucket() -> Weight {
		(36_238_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad0e47ba53c668e8be284926c7ad3e8009] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:102 w:102)
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(88_040_000 as Weight)
			// Standard Error: 44_000
			.saturating_add((10_446_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 44_000
			.saturating_add((11_238_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad0e47ba53c668e8be284926c7ad3e8009] (r:0 w:1)
	fn update_distribution_buckets_per_bag_limit() -> Weight {
		(26_628_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn update_distribution_bucket_mode() -> Weight {
		(35_165_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:2 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:1 w:1)
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight {
		(31_171_000 as Weight)
			// Standard Error: 63_000
			.saturating_add((5_492_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:2 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn invite_distribution_bucket_operator() -> Weight {
		(43_721_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn cancel_distribution_bucket_operator_invite() -> Weight {
		(39_901_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn remove_distribution_bucket_operator() -> Weight {
		(39_448_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f8903091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add16e70c3d0682e60ee3fb650594ab802] (r:1 w:0)
	fn set_distribution_bucket_family_metadata(i: u32, ) -> Weight {
		(32_713_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:1)
	fn accept_distribution_bucket_invitation() -> Weight {
		(37_640_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:0)
	fn set_distribution_operator_metadata(i: u32, ) -> Weight {
		(33_888_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: unknown [0xaff74aad5f7ed527360635c9b99b50d2b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:1 w:0)
	fn storage_operator_remark(i: u32, ) -> Weight {
		(30_528_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: unknown [0x122856f80f579448b2710bec9ee1f890b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:1 w:0)
	fn distribution_operator_remark(i: u32, ) -> Weight {
		(34_217_000 as Weight)
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
	fn update_blacklist(_i: u32, _j: u32, ) -> Weight {
		0
	}
	fn create_storage_bucket() -> Weight {
		0
	}
	fn update_storage_buckets_for_bag(_i: u32, _j: u32, ) -> Weight {
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
	fn update_distribution_buckets_for_bag(_i: u32, _j: u32, ) -> Weight {
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
	fn set_distribution_bucket_family_metadata(i: u32, ) -> Weight {
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
