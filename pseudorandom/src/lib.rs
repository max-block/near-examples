use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    // TODO: doesn't work, why???
    pub fn random_seed(&self) -> Vec<u8> {
        env::random_seed().clone()
    }

    // TODO: doesn't work, why???
    pub fn random_int(&self, max: u64) -> u64 {
        let mut base = 1_u128;
        for x in env::random_seed() {
            base *= x as u128;
        }

        (base % max as u128) as u64
    }

    pub fn random_by_timestamp_and_block(&self, max: u64) -> u64 {
        let base: u128 = (env::block_timestamp() + env::block_index()) as u128;
        (base % max as u128) as u64
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    use super::Contract;

    #[test]
    fn test_random() {
        let mut context = VMContextBuilder::new();
        context.block_index(888);
        context.block_timestamp(23423423432);
        context.random_seed(vec![7u8; 32]);

        testing_env!(context.build());
        let contract = Contract {};
        println!("random_seed: {:?}", contract.random_seed());
        println!("random_int: {}", contract.random_int(777));
        println!(
            "random_by_timestamp_and_block: {}",
            contract.random_by_timestamp_and_block(777)
        );
    }
}
