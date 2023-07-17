use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId, Balance,
};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ShopMetadata {
    pub shop_id: AccountId,
    pub name: String,
    pub decription: String,
    pub owner: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    pub product_id: String,
    pub shop_id: AccountId,
    pub name: String,
    pub price: Balance,
    pub total_supply: u64,
}