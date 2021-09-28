// This file is part of Substrate-node-template.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, 
		pallet_prelude::*, 
		traits::{Randomness,Currency,ReservableCurrency,ExistenceRequirement,},
    	sp_io::hashing::blake2_128,
	};

	use frame_system::pallet_prelude::*;
    use codec::{Encode,Decode};
	use sp_std::prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned,Bounded};

	// Kitty struct
	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16]);

	type KittyIndex = u32;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	//Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// 定义KittyIndex类型，要求实现执行的trait
		// paramter 表示可以用于函数参数传递
		// AtLeast32Bit 表示转换为u32不会造成数据丢失
		// Bounded 表示包含上界和下界
		// Default 表示有默认值
		// Copy 表示实现Copy方法
		type KittyIndex: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;

		// 创建Kitty的时候，需要质押代币
		type KittyReserve: Get<BalanceOf<Self>>;
		// 引入资产类型
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_, KittyIndex>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_prices)]
	pub type KittyPrices<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<BalanceOf<T>>,ValueQuery>;
	
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
		KittyForSale(T::AccountId, KittyIndex, Option<BalanceOf<T>>),
		KittySaleOut(T::AccountId, KittyIndex, Option<BalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		AlreadyOwned,
		MoneyNotEnough,
		NotForSale,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			};
	
			let dna = Self::random_value(&who);

			// 质押指定数量的资产，资产质押失败，报错
			T::Currency::reserve(&who, T::KittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough)?;
	
			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
	
			Owner::<T>::insert(kitty_id,Some(who.clone()));
	
			KittiesCount::<T>::put(kitty_id + 1);
	
			Self::deposit_event(Event::KittyCreate(who, kitty_id));
	
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>,new_owner: T::AccountId, kitty_id: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Owner::<T>::get(kitty_id) != None, Error::<T>::InvalidKittyIndex);

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			ensure!(new_owner != who, Error::<T>::AlreadyOwned);

			// 质押被转让人的代币
			T::Currency::reserve(&new_owner, T::KittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough)?;

			// 解押转出人的代币
			T::Currency::unreserve(&who, T::KittyReserve::get());

			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			};

			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8;16];

			for i in 0..dna_1.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}
			
			// 质押资产
			T::Currency::reserve(&who, T::KittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough)?;
			
			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			KittiesCount::<T>::put(kitty_id + 1);

			Self::deposit_event(Event::KittyCreate(who, kitty_id));

			Ok(())
		}

		// sale
		#[pallet::weight(0)]
		pub fn sale(origin: OriginFor<T>, kitty_id: KittyIndex, new_price: Option<BalanceOf<T>>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			//KittyPrice::<T>::mutate_exists(kitty_id, |price| *price = Some(new_price));
			KittyPrices::<T>::insert(kitty_id, new_price);

			Self::deposit_event(Event::KittyForSale(who, kitty_id, new_price));

			Ok(())
		}

		// buy
		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// 判断是否是该kitty的拥有者
			ensure!(Some(who.clone()) != Owner::<T>::get(kitty_id), Error::<T>::AlreadyOwned);

			// 检查kitty是否存在，并获取该kitty的owner
			let owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_price = KittyPrices::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;

			// 转质押 + 扣款
			// 对于购买者，先质押购买的和创建抵押的
			T::Currency::reserve(&who, T::KittyReserve::get() + kitty_price).map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 释放卖出者质押的代币
			T::Currency::unreserve(&owner, T::KittyReserve::get());

			// 释放购买者需要支付用来质押的代币
			T::Currency::unreserve(&who, kitty_price);
			// 转账
			T::Currency::transfer(&who, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;
			// 移除价格挂单
			KittyPrices::<T>::remove(kitty_id);
			// 转移Kitty
			Owner::<T>::insert(kitty_id, Some(who.clone()));

			Self::deposit_event(Event::KittySaleOut(who, kitty_id, Some(kitty_price)));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8;16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		#[allow(dead_code)]
		fn get_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					0
				},
			};

			Ok(kitty_id)
		}

		#[allow(dead_code)]
		fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndex, new_dna: [u8;16]) {
			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_id, Some(owner.clone()));

			KittiesCount::<T>::put(kitty_id + 1);
		}
	}
}
