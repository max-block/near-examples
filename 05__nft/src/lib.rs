use std::convert::TryFrom;

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{
    NFT_METADATA_SPEC, NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::{
    AccountId, BorshStorageKey, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::Serialize;

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    last_minted_id: u32,
    money_wallet: ValidAccountId,
    limit: u32, // how many tokens can be minted
    price: u128,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        money_wallet: ValidAccountId,
        name: String,
        symbol: String,
        limit: u32,
        price: u128,
    ) -> Self {
        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name,
            symbol,
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            last_minted_id: 0,
            money_wallet,
            limit,
            price,
        }
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens
            .mint(token_id, token_owner_id, Some(token_metadata))
    }

    #[payable]
    pub fn buy(&mut self, amount: u32) -> Vec<Token> {
        if amount + self.last_minted_id > self.limit {
            env::panic(b"not enough free tokens")
        }

        if env::attached_deposit() < amount as u128 * self.price {
            env::panic(b"not enough attached deposit")
        }

        if env::attached_deposit() > amount as u128 * self.price {
            // todo: transfer change
        }

        // Promise::new(env::signer_account_id()).transfer()
        env::log(b"z1");
        Promise::new(self.money_wallet.clone().into()).transfer(amount as u128 * self.price);
        env::log(b"z2");

        let mut result: Vec<Token> = Vec::new();
        for _ in 0..amount {
            let token_id = format!("{}", self.last_minted_id + 1);
            let title = format!("Token {}", token_id);
            let description = format!("bla bla {}", token_id);
            let issued_at = "todo".to_string();
            let metadata = TokenMetadata {
                title: Some(title),
                description: Some(description),
                media: None,
                media_hash: None,
                copies: Some(1),
                issued_at: Some(issued_at),
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            };
            result.push(self.tokens.mint(
                token_id,
                ValidAccountId::try_from(env::signer_account_id()).unwrap(),
                Some(metadata),
            ))
        }

        result
    }

    fn random_by_timestamp_and_block(&self, max: u64) -> u64 {
        let base: u128 = (env::block_timestamp() + env::block_index()) as u128;
        (base % max as u128) as u64
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
