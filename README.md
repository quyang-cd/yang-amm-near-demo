# yang-amm-near-demo
AMM demo build on top of NEAR chain.

# Instructions

## Gen two tokens on Near Testnet
We can do this follow this FT examples instruction: https://github.com/near-examples/FT

### instructions
#### token A
1 create token A account
near create-account token_a.quyang_dali.testnet --masterAccount quyang_dali.testnet --initialBalance 3
2 deploy
near deploy token_a.quyang_dali.testnet --wasmFile ./res/fungible_token.wasm
TX:https://explorer.testnet.near.org/transactions/HijvxbWpPThosPK4rqzXbsfaJ7KGwPHVbvnHNM22B3Se
3 init
near call token_a.quyang_dali.testnet new '{"owner_id": "token_a.quyang_dali.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Yang AMM Token A", "symbol": "YATA", "decimals": 8 }}' --accountId token_a.quyang_dali.testnet
TX:https://explorer.testnet.near.org/transactions/5TkHGtKJYfkXwxuvBUMEPeYvdpFVxbNXQA5SFc3cSQYh
4 check the metadata
near view token_a.quyang_dali.testnet ft_metadata
```
{
  spec: 'ft-1.0.0',
  name: 'Yang AMM Token A',
  symbol: 'YATA',
  icon: null,
  reference: null,
  reference_hash: null,
  decimals: 8
}
```
#### token B
1 create token B account
near create-account token_b.quyang_dali.testnet --masterAccount quyang_dali.testnet --initialBalance 3
2 deploy
near deploy token_b.quyang_dali.testnet --wasmFile ./res/fungible_token.wasm
TX:https://explorer.testnet.near.org/transactions/3UGFWcgJ28Uss44ccazM4gExbW1hPNs374cVUogyXLhT
3 init
near call token_b.quyang_dali.testnet new '{"owner_id": "token_b.quyang_dali.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Yang AMM Token B", "symbol": "YATB", "decimals": 8 }}' --accountId token_b.quyang_dali.testnet
TX:https://explorer.testnet.near.org/transactions/5nWuxU8XKJmtZWJAapJJhTyLYr3u3834oKNGJG3c5BH2
4 check the metadata
near view token_b.quyang_dali.testnet ft_metadata
```
{
  spec: 'ft-1.0.0',
  name: 'Yang AMM Token B',
  symbol: 'YATB',
  icon: null,
  reference: null,
  reference_hash: null,
  decimals: 8
}
```


# Test
```
cargo test -- --nocapture
```