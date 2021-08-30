use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    value: i128,
}

#[near_bindgen]
impl Contract {
    pub fn get_value(&self) -> i128 {
        self.value
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn dec(&mut self) {
        self.value -= 1;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

