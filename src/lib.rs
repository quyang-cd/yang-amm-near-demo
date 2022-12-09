mod utils;

use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseOrValue};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

use near_sdk::env::promise_result;
use utils::parse_promise_result;


#[ext_contract(ext_ft)]
pub trait FT {
    fn ft_balance_of(&mut self, account_id: AccountId) -> U128;

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct YangAMMContract {

    owner: AccountId,
    token_a: AccountId,
    token_b: AccountId,
    amount_token_a: U128,
    amount_token_b: U128,

    /**
     * FungibleTokenMetadata object including name & decimals.
     * see more https://nomicon.io/Standards/Tokens/FungibleToken/Metadata
        type FungibleTokenMetadata = {
            spec: string;
            name: string;
            symbol: string;
            icon: string|null;
            reference: string|null;
            reference_hash: string|null;
            decimals: number;
        }
     */
    metadata_token_a: Option<FungibleTokenMetadata>,
    metadata_token_b: Option<FungibleTokenMetadata>,

}

#[ext_contract(ext_self_metadata_trait)]
pub trait MetadataTrait {
    fn init_token_metadata(&mut self) -> PromiseOrValue<U128>;
}


// private methods for YangAMMContract
#[near_bindgen]
impl YangAMMContract {

    #[private]
    pub fn init_token_metadata(&mut self) {

        assert_eq!(env::promise_results_count(), 2, "Need medatas for token A & B.");

        let metadata_a = parse_promise_result::<FungibleTokenMetadata>(&promise_result(0));
        if metadata_a.is_some() {
            self.metadata_token_a = metadata_a;
        } else {
            env::panic_str("Err when querying token A metadata.");
        }

        let metadata_b = parse_promise_result::<FungibleTokenMetadata>(&promise_result(1));
        if metadata_b.is_some() {
            self.metadata_token_b = metadata_b;
        } else {
            env::panic_str("Err when querying token B metadata.");
        }

    }

}

// init & pub methods for YangAMMContract
#[near_bindgen]
impl YangAMMContract {

    #[init]
    pub fn new(owner: AccountId, token_a: AccountId, token_b: AccountId) -> Self {

        ext_ft::ext(token_a.clone()).ft_metadata()
        .and(
            ext_ft::ext(token_a.clone()).ft_metadata()
        ).then(
            ext_self_metadata_trait::ext(env::current_account_id()).init_token_metadata()
        );

        Self {
            owner,
            token_a,
            token_b,
            amount_token_a: U128(0),
            amount_token_b: U128(0),
            metadata_token_a: None,
            metadata_token_b: None,
        }

    }

    /*
    This AMM contract received any tokens from anyone will execute this method
    */
    pub fn ft_on_transfer(
        mut self,
        sender_id: AccountId,
        amount: U128,
        _msg: String,
    ) -> PromiseOrValue<U128> {
        let account_token_a = self.token_a.clone();
        let account_token_b = self.token_b.clone();
        if env::predecessor_account_id() != account_token_a
            && env::predecessor_account_id() != account_token_b
        {
            near_sdk::env::panic_str("Yant AMM contract do not support this token for now!");
        }
        
        if sender_id == self.owner {
            // owner of this contract deposited token a or b. the K will be changed
            // k = amount(a) * amount(b)
            match env::predecessor_account_id() {
                account_token_a => {
                    self.amount_token_a = U128(self.amount_token_a.0 + amount.0)
                },
                account_token_b => {
                    self.amount_token_b = U128(self.amount_token_b.0 + amount.0)
                },
                _ => env::panic_str("Unsupported token"),
            }
            return PromiseOrValue::Value(U128(0));
        }
        return PromiseOrValue::Value(U128(0));
    }

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn yang_amm_contract_test() {
        // Get Alice as an account ID
        let owner_account_id = AccountId::new_unchecked("quyang_dali.testnet".to_string());
        let token_a_account_id = AccountId::new_unchecked("token_a.quyang_dali.testnet".to_string());
        let token_b_account_id = AccountId::new_unchecked("token_b.quyang_dali.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(owner_account_id.clone());
        testing_env!(context.build());

        // test metadata of tokens
        let contract = YangAMMContract::new(owner_account_id, token_a_account_id.clone(), token_b_account_id.clone());
        assert_eq!(contract.token_a, token_a_account_id.clone());
        assert_eq!(contract.token_b, token_b_account_id.clone());
        assert_eq!(contract.amount_token_a, U128(0));
        assert_eq!(contract.amount_token_b, U128(0));
        // assert_eq!(contract.metadata_token_a.unwrap().decimals, 8);
        // assert_eq!(contract.metadata_token_b.unwrap().decimals, 8);
    }

}