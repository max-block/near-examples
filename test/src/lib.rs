use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    value: i128,
    history: Vector<Action>,
}

impl Default for Contract {
    fn default() -> Self {
        Self { value: 0, history: Vector::new(b"h") }
    }
}


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Action {
    user: AccountId,
    operation: String,
    param: i128,
}

impl Contract {
    pub fn get_history(&self) -> Vec<Action> {
        self.history.to_vec()
    }

    pub fn add_history(&mut self) {
        self.history.push(&Action { user: env::signer_account_id(), operation: "add".to_string(), param: 9 });
    }
}