// This file is part of Substrate-node-template.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, traits::{OnFinalize, OnInitialize}};

fn run_to_block(n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

// 测试创建一个 Kitty
#[test]
fn create_kitty_work() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		// 检查事件
		System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(1, 1)));
	});
}

// 测试创建一个kitty失败，因为没有质押的资金
#[test]
fn create_kitty_failed_when_not_enough_money() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(KittiesModule::create(Origin::signed(9)), Error::<Test>::MoneyNotEnough);
	});
}

// 测试转让kitty成功
#[test]
fn transfer_kitty_works() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesCount::<Test>::get(), Some(2));

		// 检查event
		System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(1, 1)));
	});
}

// 测试转让kitty失败, not exists of kitty
#[test]
fn transfer_kitty_failed_when_not_exists() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(KittiesModule::transfer(Origin::signed(1), 2, 0), Error::<Test>::InvalidKittyIndex);
	})
}

// 测试转让kitty失败，not owner of kitty
#[test]
fn transfer_kitty_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 3, 0), Error::<Test>::InvalidKittyIndex);
	})
}

#[test]
// 测试转让kitty失败，not transfer to self
fn transfer_kitty_failed_when_not_self() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(KittiesModule::transfer(Origin::signed(1), 1, 0), Error::<Test>::InvalidKittyIndex);
	})
}

// breed_kitty_works
#[test]
fn breed_kitty_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesCount::<Test>::get(), Some(3));

		assert_eq!(Owner::<Test>::get(1), Some(1));
		assert_eq!(Owner::<Test>::get(2), Some(1));
		
		// 检查事件
		System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(1, 2)));
	});
}

#[test]
fn breed_kitty_failed_invalid_kittyindex() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 0, 1),
			Error::<Test>::InvalidKittyIndex,
		);
	})
}

#[test]
fn breed_kitty_failed_invalid_same_parent_index() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 1, 1),
			Error::<Test>::SameParentIndex,
		);
	})
}

#[test]
fn sale_kitty_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(1, 1)));
	})
}

#[test]
fn sale_kitty_failed_not_owner() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::sale(Origin::signed(1), 0, Some(5_000)),
			Error::<Test>::NotOwner,
		);
	})
}

#[test]
fn buy_kitty_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_eq!(Owner::<Test>::get(1), Some(1));

		assert_eq!(KittyPrices::<Test>::get(0), None);

		System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(1, 1)));
	})
}

#[test]
fn buy_kitty_failed_invalid_kittyindex() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::buy(Origin::signed(1), 99),
			Error::<Test>::InvalidKittyIndex,
		);
	})
}	

#[test]
fn buy_kitty_failed_not_for_sale() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(2), 0),
			Error::<Test>::NotForSale,
		);
	})
}

#[test]
fn buy_kitty_failed_already_owned() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(1), 0), 
			Error::<Test>::AlreadyOwned,
		);
	})
}
