#![allow(dead_code)]
pub mod metadata;
pub mod ecommerce;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, env};
use near_sdk::collections::{UnorderedMap};

use metadata::{ShopMetadata, Product};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  owner_id: AccountId,
  shops: UnorderedMap<AccountId, ShopMetadata>,
  products: UnorderedMap<u64, Product>,
}

// Define the default, which automatically initializes the contract

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::signer_account_id(),   
            shops: UnorderedMap::new(b"s".to_vec()),
            products: UnorderedMap::new(b"p".to_vec()),
        }
    }

    pub fn view_all_shop (&self) -> Vec<ShopMetadata>{
        self.shops.values().collect()
    }
}
