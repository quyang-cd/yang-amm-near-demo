
use near_sdk::json_types::U128;
use near_sdk::ONE_YOCTO;
use near_units::parse_near;
use workspaces::{Account, AccountId, Contract,DevNetwork, Worker};
use workspaces::operations::Function;
use workspaces::result::ValueOrReceiptId;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

async fn register_user(
    contract: &Contract,
    account_id: &AccountId,
) -> anyhow::Result<()> {
    let res = contract
        .call("storage_deposit")
        .args_json((account_id, Option::<bool>::None))
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await?;
    assert!(res.is_success());

    Ok(())
}

async fn init(
    worker: &Worker<impl DevNetwork>,
    initial_balance: U128,
) -> anyhow::Result<(Account, Account, Contract, Contract, Contract)> {
    let token_a_contract =
        worker.dev_deploy(include_bytes!("../res/fungible_token.wasm")).await?;

    let res = token_a_contract
        .call("new_default_meta")
        .args_json((token_a_contract.id(), initial_balance))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    let token_b_contract = 
        worker.dev_deploy(include_bytes!("../res/fungible_token.wasm")).await?;

    let res = token_b_contract
        .call("new_default_meta")
        .args_json((token_b_contract.id(), initial_balance))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    let alice = token_a_contract
        .as_account()
        .create_subaccount("alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;
    register_user(&token_a_contract, alice.id()).await?;

    let res = token_a_contract
        .call("storage_deposit")
        .args_json((alice.id(), Option::<bool>::None))
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    let bob = token_b_contract
        .as_account()
        .create_subaccount("bob")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;
    register_user(&token_b_contract, bob.id()).await?;

    let res = token_b_contract
        .call("storage_deposit")
        .args_json((bob.id(), Option::<bool>::None))
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());


    let amm_contract = worker.dev_deploy(include_bytes!("../res/yang_amm.wasm")).await?;

    let res = amm_contract
        .call("new")
        .args_json((alice.id(), token_a_contract.id(), token_b_contract.id(),))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    return Ok((alice, bob, token_a_contract, token_b_contract, amm_contract));
}


#[tokio::test]
async fn amm_tests() -> anyhow::Result<()> {

    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await?;
    let (alice, bob, token_a_contract, token_b_contract, amm_contract) = init(&worker, initial_balance).await?;

    let res = amm_contract.call("tokens_metadata").view().await?.json()?;
    Ok(())
}