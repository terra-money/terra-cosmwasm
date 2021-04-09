use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MsgSwap {
        offer_coin: Coin,
        ask_denom: String,
    },
    MsgSwapSend {
        offer_coin: Coin,
        ask_denom: String,
        recipient: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    TaxRate {},
    TaxCap {
        denom: String,
    },
    ExchangeRates {
        base_denom: String,
        quote_denoms: Vec<String>,
    },
}
