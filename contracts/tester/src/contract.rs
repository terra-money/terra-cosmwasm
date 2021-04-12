use cosmwasm_std::{
    to_binary, Addr, Coin, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, TerraMsgWrapper, TerraQuerier,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<TerraMsgWrapper>> {
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TerraMsgWrapper>, StdError> {
    match msg {
        ExecuteMsg::MsgSwap {
            offer_coin,
            ask_denom,
        } => execute_msg_swap(deps, env, info, offer_coin, ask_denom, None),
        ExecuteMsg::MsgSwapSend {
            offer_coin,
            ask_denom,
            recipient,
        } => execute_msg_swap(deps, env, info, offer_coin, ask_denom, Some(recipient)),
    }
}

pub fn execute_msg_swap(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    offer_coin: Coin,
    ask_denom: String,
    recipient: Option<Addr>,
) -> StdResult<Response<TerraMsgWrapper>> {
    let msg = if let Some(recipient) = recipient {
        create_swap_send_msg(recipient.to_string(), offer_coin, ask_denom)
    } else {
        create_swap_msg(offer_coin, ask_denom)
    };

    let res = Response {
        messages: vec![msg],
        ..Response::default()
    };
    Ok(res)
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
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

pub fn query_swap(deps: Deps, offer_coin: Coin, ask_denom: String) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate(deps: Deps) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap(deps: Deps, denom: String) -> StdResult<TaxCapResponse> {
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
