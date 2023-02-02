
//! Autogenerated weights for `dex_router`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/gpsh9wvfcrwyck2nw62gpkqhf0bhc0cw-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `dex_router`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> dex_router::WeightInfo for WeightInfo<T> {
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: DexRouter DexRoutes (r:2 w:1)
	fn update_route() -> Weight {
		// Minimum execution time: 49_542 nanoseconds.
		Weight::from_ref_time(57_834_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DexRouter DexRoutes (r:2 w:0)
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: Tokens Accounts (r:13 w:13)
	// Storage: System Account (r:4 w:0)
	// Storage: Pablo PriceCumulativeState (r:4 w:4)
	fn swap() -> Weight {
		// Minimum execution time: 297_627 nanoseconds.
		Weight::from_ref_time(323_960_000 as u64)
			.saturating_add(T::DbWeight::get().reads(27 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: Tokens Accounts (r:13 w:13)
	// Storage: System Account (r:4 w:0)
	// Storage: Pablo PriceCumulativeState (r:4 w:4)
	fn buy() -> Weight {
		// Minimum execution time: 402_835 nanoseconds.
		Weight::from_ref_time(411_461_000 as u64)
			.saturating_add(T::DbWeight::get().reads(26 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 146_834 nanoseconds.
		Weight::from_ref_time(170_459_000 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 73_084 nanoseconds.
		Weight::from_ref_time(79_876_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
