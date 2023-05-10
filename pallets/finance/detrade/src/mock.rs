
#![cfg(test)]
use crate as trade_finance;

use trade_finance::*;


use sp_core::H256;
use sp_runtime::{
	testing::{Header},
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::{
	traits::{ConstU32, ConstU64},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;


frame_support::construct_runtime!(
	pub enum Test where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        TradeFinance: trade_finance::{Pallet, Call, Storage, Event<T>},
	}
);


impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

pub fn new_test_ext() ->sp_io::TestExternalities{
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	t.into()
}
