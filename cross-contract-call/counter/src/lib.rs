use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Action {
    user: AccountId,
    operation: String,
    param: i128,
    timestamp: u64,
}

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

#[near_bindgen]
impl Contract {
    pub fn value(&self) -> i128 {
        self.value
    }

    pub fn history(&self) -> Vec<Action> {
        self.history.to_vec()
    }

    pub fn action(&mut self, operation: String, param: i128) -> i128 {
        env::log(format!("action call: operation={}, param={}", operation, param).as_bytes());
        match operation.as_str() {
            "add" => {
                self.value += param;
            }
            "sub" => {
                self.value -= param;
            }
            _ => {
                env::panic(b"unsupported operation");
            }
        }
        self.history.push(&Action {
            user: env::signer_account_id(),
            operation,
            param,
            timestamp: env::block_timestamp(),
        });
        self.value
    }

    #[payable]
    // You can reset only if you pay 0.5 NEAR
    pub fn reset(&mut self) {
        env::log(format!("reset for {}", env::attached_deposit()).as_bytes());
        if env::attached_deposit() < 500000000000000000000000 {
            env::panic(b"Pay at least 0.5 NEAR for this action.")
        }
        self.value = 0;
        self.history.clear();
    }
}
