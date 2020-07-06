use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, CosmosMsg, HumanAddr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// TerraMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct TerraMsgWrapper {
    pub route: String,
    pub msg_data: TerraMsg,
}

// this is a helper to be able to return these as CosmosMsg easier
impl Into<CosmosMsg<TerraMsgWrapper>> for TerraMsgWrapper {
    fn into(self) -> CosmosMsg<TerraMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerraMsg {
    Swap {
        trader: HumanAddr,
        offer_coin: Coin,
        ask_denom: String,
    },
    SwapSend {
        from_address: HumanAddr,
        to_address: HumanAddr,
        offer_coin: Coin,
        ask_denom: String,
    },
}

// create_swap_msg returns wrapped swap msg
pub fn create_swap_msg(
    trader: HumanAddr,
    offer_coin: Coin,
    ask_denom: String,
) -> CosmosMsg<TerraMsgWrapper> {
    TerraMsgWrapper {
        route: "market".to_string(),
        msg_data: TerraMsg::Swap {
            trader,
            offer_coin,
            ask_denom,
        },
    }
    .into()
}

// create_swap_send_msg returns wrapped swap send msg
pub fn create_swap_send_msg(
    from_address: HumanAddr,
    to_address: HumanAddr,
    offer_coin: Coin,
    ask_denom: String,
) -> CosmosMsg<TerraMsgWrapper> {
    TerraMsgWrapper {
        route: "market".to_string(),
        msg_data: TerraMsg::SwapSend {
            from_address,
            to_address,
            offer_coin,
            ask_denom,
        },
    }
    .into()
}
