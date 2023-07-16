#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
  fn default() -> Self {
    Self { message: "Hello, {{project-name}}".to_string() }
  }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  // get the message
  pub fn get_message(&self) -> String {
    self.message.clone()
  }

  // Call this method to change the message b
  pub fn set_message(&mut self, message: u32) {
    self.message = message.to_string();
  }
}
