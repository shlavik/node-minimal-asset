use crate as pallet_asset;
pub use frame::{
	deps::{frame_support::runtime, frame_system},
	runtime::prelude::*,
	testing_prelude::*,
};

#[runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask
	)]
	pub struct Mock;

	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Mock>;

	#[runtime::pallet_index(1)]
	pub type Asset = pallet_asset::Pallet<Mock>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Mock {
	type Block = MockBlock<Self>;
	type AccountId = u64;
}

impl pallet_asset::Config for Mock {
	type RuntimeEvent = RuntimeEvent;
}
