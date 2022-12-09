
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseOrValue};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct YangAMMContract {

    owner: AccountId,
    token_a: AccountId,
    token_b: AccountId,

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
    pub fn new(owner: String, token_a: String, token_b: String) -> Self {

        ext_ft::ext(AccountId::from_str(&token_a).unwrap()).ft_metadata()
        .and(
            ext_ft::ext(AccountId::from_str(&token_b).unwrap()).ft_metadata()
        ).then(
            ext_self_metadata_trait::ext(env::current_account_id()).init_token_metadata()
        );

        Self {
            owner: AccountId::from_str(&owner).unwrap(),
            token_a: AccountId::from_str(&token_a).unwrap(),
            token_b: AccountId::from_str(&token_b).unwrap(),
        }

    }

    #[result_serializer(borsh)]
    pub fn metadata_tokens(self) -> MetadataTokens {
        return MetadataTokens {
            metadata_token_a: self.metadata_token_a.unwrap(),
            metadata_token_b: self.metadata_token_b.unwrap(),
        };
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
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let testnet = AccountId::new_unchecked("testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        // Set up contract object and call the new method
        let mut contract = YangAMMContract::new("alice.testnet", "a.");

    }

}