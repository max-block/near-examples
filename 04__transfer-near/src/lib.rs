use near_sdk::{AccountId, env, near_bindgen, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn deposit(&mut self) {
        env::log(format!("deposit: {}", env::attached_deposit()).as_bytes());
    }

    pub fn send(&self, amount: U128, recipients: Vec<AccountId>) {
        for recipient in recipients {
            Promise::new(recipient.clone()).transfer(amount.0);
            env::log(format!("send {} tokens to {}", amount.0, recipient).as_bytes());
        }
    }
}
