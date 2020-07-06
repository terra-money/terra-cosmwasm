# Terra Binding for CosmWasm Contracts

## How to use querier
```
use cosmwasm_std::Coin;
use terra_bindings::TerraQuerier;

let res: Coin = TerraQuerier::new(&deps.querier).query_swap(offer.clone(), ask)?;
```

## How to use msg creator
```
use cosmwasm_std::CosmosMsg;
use terra_bindings::{create_swap_msg, TerraMsgWrapper};

let msg: CosmosMsg<TerraMsgWrapper> = create_swap_msg(contract_addr, offer_coin, ask_denom);
```