
//! Autogenerated weights for `scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/gpsh9wvfcrwyck2nw62gpkqhf0bhc0cw-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 6_208 nanoseconds.
		Weight::from_ref_time(18_333_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 5_417 nanoseconds.
		Weight::from_ref_time(7_083_500 as u64)
			// Standard Error: 132_994
			.saturating_add(Weight::from_ref_time(890_420 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 13_250 nanoseconds.
		Weight::from_ref_time(19_625_000 as u64)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 26_542 nanoseconds.
		Weight::from_ref_time(30_109_308 as u64)
			// Standard Error: 33
			.saturating_add(Weight::from_ref_time(610 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 14_417 nanoseconds.
		Weight::from_ref_time(21_917_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 12_792 nanoseconds.
		Weight::from_ref_time(18_500_000 as u64)
	}
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 9_292 nanoseconds.
		Weight::from_ref_time(20_334_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 8_500 nanoseconds.
		Weight::from_ref_time(9_376_000 as u64)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 21_792 nanoseconds.
		Weight::from_ref_time(24_021_500 as u64)
			// Standard Error: 309_485
			.saturating_add(Weight::from_ref_time(1_107_561 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 22_833 nanoseconds.
		Weight::from_ref_time(23_855_265 as u64)
			// Standard Error: 64_812
			.saturating_add(Weight::from_ref_time(936_234 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 26_708 nanoseconds.
		Weight::from_ref_time(29_833_000 as u64)
			// Standard Error: 168_957
			.saturating_add(Weight::from_ref_time(941_336 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 24_375 nanoseconds.
		Weight::from_ref_time(27_270_142 as u64)
			// Standard Error: 304_700
			.saturating_add(Weight::from_ref_time(1_063_357 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
