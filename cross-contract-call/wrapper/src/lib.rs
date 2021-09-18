use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, Promise, PromiseResult,
};
use near_sdk::{ext_contract, near_bindgen, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contract_address: String,
}

#[ext_contract(ext_self)]
pub trait SelfContract {
    fn value_callback(&self, param: i128);
}

#[ext_contract(ext_counter)]
pub trait ExtCounter {
    fn value(&self) -> i128;
    fn action(&mut self, operation: String, param: i128) -> i128;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(contract_address: String) -> Self {
        Self { contract_address }
    }

    pub fn add_if_even(&self, param: i128) -> Promise {
        ext_counter::value(&self.contract_address, 0, 30000000000000).then(ext_self::value_callback(
            param,
            &env::current_account_id(),
            0,
            30000000000000,
        ))
    }

    pub fn value_callback(&self, param: i128) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "promise was failed".to_string(),
            PromiseResult::Successful(result) => {
                let current_value = near_sdk::serde_json::from_slice::<i128>(&result).unwrap();
                if current_value % 2 == 0 {
                    ext_counter::action("add".to_string(), param, &self.contract_address, 0, 3000000000000);
                    "even, add was requested".to_string()
                } else {
                    "odd".to_string()
                }
            }
        }
    }
}
