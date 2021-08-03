# Terra Bindings for CosmWasm
​
This crate provides Terra-specific bindings to enable your CosmWasm smart contracts to interact with the Terra blockchain by exposing messages and queriers that can be emitted and used from within your contract.
​
## Installation
​
Add the following to your smart contract's `Cargo.toml`:
​
```toml
[dependencies]
terra-cosmwasm = { version = "2.2" }
```
​
## Contents
​
Currently, the Terra bindings include:
​
- Query support for:
  - Market
    - swap rate between 2 currencies at market price
  - Treasury
    - current tax cap for a denomination
    - current tax rate 
  - Oracle
    - exchange rates for the given base_denom / quote_denoms
​
- Messages
  - `MsgSwap`
  - `MsgSwapSend`
​
## Usage
​
### Querying
​
In order to use the query functions enabled by the bindings, create a `TerraQuerier` instance within your contract logic -- in either `init()`, `handle()`, or `query()` entrypoints. You can access all the enabled queries through this object.
​
```rust
// src/contract.rs
use cosmwasm_std::Coin;
use terra_cosmwasm::{ TerraQuerier, SwapResponse, TaxRateResponse, TaxCapResponse, ExchangeRatesResponse };
​
...
​
// handler
pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    offer: &Coin
) -> StdResult<HandleResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let swap_rate: SwapResponse = querier.query_swap(offer.clone(), "uusd")?;
    let tax_cap: TaxCapResponse = querier.query_tax_cap("usdr")?;
    let tax_rate: TaxRateResponse = querier.query_tax_rate()?;
    let exchange_rates: ExchangeRatesResponse = querier.query_exchange_rates("uusd", vec!["uluna", "ukrw"])?;
    ...
}
```
​
## Creating Messages
​
**NOTE:** The Terra bindings do not cover messages that have already been implemented by the CosmWasm team, such as staking-related messages and fundamental ones like `MsgSend`.
​
You may want your contract to perform messages such as `MsgSwap` and `MsgSwapSend` operations at the end of its execution. To do this, create a message using the predefined functions:
​
- `create_swap_msg`
- `create_swap_send_msg`
​
And add it to the vector of `messages` in your `HandleResponse` before you return `Ok`.
​
```rust
use cosmwasm_std::CosmosMsg;
use terra_cosmwasm::{create_swap_msg, TerraMsgWrapper};
​
...
​
pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    offer: &Coin
) -> StdResult<HandleResponse<TerraMsgWrapper>> {
    ...
​
    let msg: CosmosMsg<TerraMsgWrapper> = create_swap_msg(contract_addr, offer_coin, ask_denom);
    let res = HandleResponse {
        messages: vec![msg],
        log: vec![],
        data: None
    };
    Ok(res)
}
```