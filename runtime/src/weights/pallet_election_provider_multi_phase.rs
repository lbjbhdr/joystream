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

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --base-path=/mnt/disks/local-ssd/
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/frame-weight-template.hbs
// --output=./../runtime/src/weights/pallet_election_provider_multi_phase.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub use pallet_election_provider_multi_phase::weights::WeightInfo;

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Storage: Staking ForceEra (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	fn on_initialize_nothing() -> Weight {
		(23_280_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_signed() -> Weight {
		(18_320_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_unsigned() -> Weight {
		(17_310_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	fn finalize_signed_phase_accept_solution() -> Weight {
		(41_330_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn finalize_signed_phase_reject_solution() -> Weight {
		(30_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	fn create_snapshot_internal(v: u32, t: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((286_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 33_000
			.saturating_add((371_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn elect_queued(a: u32, _d: u32, ) -> Weight {
		(215_156_000 as Weight)
			// Standard Error: 53_000
			.saturating_add((1_186_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	fn submit() -> Weight {
		(66_330_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((989_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 38_000
			.saturating_add((651_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 63_000
			.saturating_add((9_673_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 95_000
			.saturating_add((2_907_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	fn feasibility_check(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((2_823_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 140_000
			.saturating_add((4_501_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 233_000
			.saturating_add((10_729_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
}
