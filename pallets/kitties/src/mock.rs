// This file is part of Substrate-node-template.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate as pallet_kitties;
use sp_core::H256;
use frame_support::parameter_types;
use frame_system as system;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		//RandomnessCollectiveFlip: pallet_randomness_collective_filp::{Pallet, Storage},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		KittiesModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::AllowAll;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}


parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

//impl pallet_randomness_collective_flip::Config for Test {}
impl pallet_randomness_collective_flip::Config for Test {}

parameter_types! {
	pub const KittyReserve: u64 = 5_000;
}

impl pallet_kitties::Config for Test {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
	type KittyReserve = KittyReserve;
	type KittyIndex = u32;
	type Currency = Balances;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100000000), (2, 200000000), (3, 300000000)],
	}.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
