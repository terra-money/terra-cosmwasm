use cosmwasm_std::{
    to_binary, Binary, Coin, Env, HandleResponse, HumanAddr, InitResponse,
    StdResult, Deps, DepsMut, MessageInfo, StdError,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, TerraMsgWrapper, TerraQuerier,
};

pub fn init(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<InitResponse, StdError> {
    Ok(InitResponse::default())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<TerraMsgWrapper>, StdError> {
//) -> Result<HandleResponse<TerraMsgWrapper>, StdError> {
    match msg {
        
        HandleMsg::MsgSwap {
            offer_coin,
            ask_denom,
        } => handle_msg_swap(deps, env, info, offer_coin, ask_denom, None),
        HandleMsg::MsgSwapSend {
            offer_coin,
            ask_denom,
            recipient,
        } => handle_msg_swap(deps, env, info, offer_coin, ask_denom, Some(recipient)),
        
    }
}

pub fn handle_msg_swap(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    offer_coin: Coin,
    ask_denom: String,
    recipient: Option<HumanAddr>,
) -> Result<HandleResponse<TerraMsgWrapper>, StdError> {
    //) -> Result<HandleResponse<TerraMsgWrapper>, StdError> {
    let msg = if let Some(recipient) = recipient {
        create_swap_send_msg(info.sender, recipient, offer_coin, ask_denom)
    } else {
        create_swap_msg(info.sender, offer_coin, ask_denom)
    };

    let res = HandleResponse {
        //messages: vec![msg],
        messages: vec![msg],
        attributes: vec![],
        data: None,
    };
    Ok(res)
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, StdError> {
    match msg {
        QueryMsg::Swap {
            offer_coin,
            ask_denom,
        } => to_binary(&query_swap(deps, offer_coin, ask_denom)?),
        QueryMsg::TaxRate {} => to_binary(&query_tax_rate(deps)?),
        QueryMsg::TaxCap { denom } => to_binary(&query_tax_cap(deps, denom)?),
        QueryMsg::ExchangeRates {
            base_denom,
            quote_denoms,
        } => to_binary(&query_exchange_rates(deps, base_denom, quote_denoms)?), 
    }
}

pub fn query_swap(
    deps: Deps,
    offer_coin: Coin,
    ask_denom: String,
) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate(
    deps: Deps,
) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap(
    deps: Deps,
    denom: String,
) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxCapResponse = querier.query_tax_cap(denom)?;

    Ok(res)
}

pub fn query_exchange_rates(
    deps: Deps,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;

    Ok(res)
} 
