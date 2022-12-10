mod utils;

use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

use near_sdk::env::promise_result;
use utils::parse_promise_result;
use near_contract_standards::fungible_token::core::ext_ft_core;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
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
        let token =  env::predecessor_account_id();
        let account_token_a = self.token_a.clone();
        let account_token_b = self.token_b.clone();
        if token != account_token_a
            && token != account_token_b
        {
            near_sdk::env::panic_str("Yant AMM contract do not support this token for now!");
        }
        let balance_token_a: u128 = self.amount_token_a.0;
        let balance_token_b: u128 = self.amount_token_b.0;
        let balance_amount: u128 = amount.0;
        // for contract owner, that means deposit to change K.
        if sender_id == self.owner {
            // owner of this contract deposited token a or b. the K will be changed
            // k = amount(a) * amount(b)
            match token {
                account_token_a => {
                    self.amount_token_a = U128(balance_token_a + balance_amount)
                },
                account_token_b => {
                    self.amount_token_b = U128(balance_token_b + balance_amount)
                },
                _ => env::panic_str("Unsupported token"),
            }
            return PromiseOrValue::Value(U128(0));
        }
        // for any others, that means swap a from b or b from a.
        match token {
            account_token_a => {
                let amount_token_b_for_swap = U128(balance_token_b - (balance_token_b / balance_token_a) * (balance_token_a - balance_amount));
                ext_ft_core::ext(account_token_b).with_attached_deposit(1).ft_transfer(sender_id, amount_token_b_for_swap, None);
            },
            account_token_b => {
                let amount_token_a_for_swap = U128(balance_token_a - (balance_token_a / balance_token_b) * (balance_token_b - balance_amount));
                ext_ft_core::ext(account_token_a).with_attached_deposit(1).ft_transfer(sender_id, amount_token_a_for_swap, None);
            },
            _ => env::panic_str("Unsupported token"),
        }
        return PromiseOrValue::Value(U128(0));
    }

}