#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageMap, StorageU256, StorageAddress, StorageBool};

#[storage]
#[entrypoint]  // Add this line!
pub struct DatasetRegistry {
    dataset_count: StorageU256,
    dataset_exists_flag: StorageMap<U256, StorageBool>,
    dataset_companies: StorageMap<U256, StorageAddress>,
    dataset_prices_eth: StorageMap<U256, StorageU256>,
    dataset_prices_usdc: StorageMap<U256, StorageU256>,
}

#[public]
impl DatasetRegistry {
    pub fn register_dataset(
        &mut self,
        price_eth: U256,
        price_usdc: U256,
    ) -> U256 {
        let dataset_id = self.dataset_count.get();
        let sender = self.vm().msg_sender();

        self.dataset_exists_flag.insert(dataset_id, true);
        self.dataset_companies.insert(dataset_id, sender);
        self.dataset_prices_eth.insert(dataset_id, price_eth);
        self.dataset_prices_usdc.insert(dataset_id, price_usdc);
        self.dataset_count.set(dataset_id + U256::from(1));

        dataset_id
    }

    pub fn get_dataset(&self, dataset_id: U256) -> (Address, U256, U256) {
        let company = self.dataset_companies.get(dataset_id);
        let price_eth = self.dataset_prices_eth.get(dataset_id);
        let price_usdc = self.dataset_prices_usdc.get(dataset_id);
        
        (company, price_eth, price_usdc)
    }

    pub fn get_dataset_count(&self) -> U256 {
        self.dataset_count.get()
    }

    pub fn dataset_exists(&self, dataset_id: U256) -> bool {
        self.dataset_exists_flag.get(dataset_id)
    }
}