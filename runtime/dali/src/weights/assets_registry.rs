
//! Autogenerated weights for `assets_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=assets_registry
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `assets_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> assets_registry::WeightInfo for WeightInfo<T> {
	// Storage: AssetsRegistry LocalAdmin (r:0 w:1)
	fn set_local_admin() -> Weight {
		(9_948_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry ForeignAdmin (r:0 w:1)
	fn set_foreign_admin() -> Weight {
		(9_777_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:0)
	// Storage: AssetsRegistry AssetsMappingCandidates (r:1 w:1)
	fn approve_assets_mapping_candidate() -> Weight {
		(22_662_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignAssetMetadata (r:0 w:1)
	fn set_metadata() -> Weight {
		(17_662_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}