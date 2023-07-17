// pub mod metadata;
use crate::*;
use near_sdk::AccountId;
use near_sdk::Balance;
use near_sdk::Promise;
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::env;
use crate::Contract;


pub trait ImplementECommerce {
    fn create_shop(&mut self, shop_id: AccountId, name: String, decription: String);
    fn create_product(
        &mut self,
        product_id: String,
        shop_id: AccountId,
        name: String,
        price: Balance,
        total_supply: u64,
    );
    fn view_all_products(&self) -> Vec<Product>;
    fn view_all_products_per_shop(&self, shop_id: AccountId) -> Vec<Product>;
    fn view_product_by_id(&self, product_id: String) -> Option<Product>;
    fn payment(&mut self, product_id: String, amount: u64) -> Promise;
}

#[near_bindgen]
impl ImplementECommerce for Contract {
    fn create_shop(&mut self, shop_id: AccountId, name: String, decription: String) {
        assert!(self.shops.get(&shop_id).is_none(), "Shop already exists");
        let shop = ShopMetadata {shop_id: 
            shop_id.clone(), name, decription};
        self.shops.insert(&shop_id, &shop);
    }

    fn create_product(
        &mut self,
        product_id: String,
        shop_id: AccountId,
        name: String,
        price: Balance,
        total_supply: u64,
    ) {
        assert!(self.products.get(&product_id).is_none(), "Product already exists");
        assert!(self.shops.get(&shop_id).is_some(), "Shop does not exist");

        let product = Product{product_id: product_id.clone(), shop_id, name, price, total_supply};
        self.products.insert(&product_id, &product);
    }

    fn view_all_products(&self) -> Vec<Product> {
        self.products.values().collect()
    }

     fn view_all_products_per_shop(&self, shop_id: AccountId) -> Vec<Product> {
        self.products
            .values()
            .filter(|product| product.shop_id == shop_id)
            .collect()
    }

     fn view_product_by_id(&self, product_id: String) -> Option<Product> {
        self.products.get(&product_id)
    }

    #[payable]
     fn payment(&mut self, product_id: String, amount: u64) -> Promise {
        let mut product = self.products.get(&product_id).expect("Product does not exist");
        assert!(product.total_supply > 0, "Product out of stock");

        product.total_supply -= amount;
        self.products.insert(&product_id, &product);
        let buyer = env::signer_account_id();
        let price = product.price;
        Promise::new(buyer).transfer(price)
    }
}
