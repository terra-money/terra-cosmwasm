use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::TerraRoute;
use cosmwasm_std::{Coin, CosmosMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// TerraMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct TerraMsgWrapper {
    pub route: TerraRoute,
    pub msg_data: TerraMsg,
}

// define trait bound
impl cosmwasm_std::CustomMsg for TerraMsgWrapper {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<TerraMsgWrapper> for CosmosMsg<TerraMsgWrapper> {
    fn from(original: TerraMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerraMsg {
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    SwapSend {
        to_address: String,
        offer_coin: Coin,
        ask_denom: String,
    },
}

// create_swap_msg returns wrapped swap msg
pub fn create_swap_msg(offer_coin: Coin, ask_denom: String) -> CosmosMsg<TerraMsgWrapper> {
    TerraMsgWrapper {
        route: TerraRoute::Market,
        msg_data: TerraMsg::Swap {
            offer_coin,
            ask_denom,
        },
    }
    .into()
}

// create_swap_send_msg returns wrapped swap send msg
pub fn create_swap_send_msg(
    to_address: String,
    offer_coin: Coin,
    ask_denom: String,
) -> CosmosMsg<TerraMsgWrapper> {
    TerraMsgWrapper {
        route: TerraRoute::Market,
        msg_data: TerraMsg::SwapSend {
            to_address,
            offer_coin,
            ask_denom,
        },
    }
    .into()
}
