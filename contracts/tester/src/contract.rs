use cosmwasm_std::{
    to_binary, Addr, Coin, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, entry_point
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ContractInfoResponse, ExchangeRatesResponse,
    SwapResponse, TaxCapResponse, TaxRateResponse, TerraMsgWrapper, TerraQuerier,
    TerraQueryWrapper,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut<TerraQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<TerraMsgWrapper>> {
    Ok(Response::new())
}

#[entry_point]
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

    Ok(Response::new().add_message(msg))
}

#[entry_point]
pub fn query(deps: Deps<TerraQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
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
        QueryMsg::ContractInfo { contract_address } => {
            to_binary(&query_contract_info(deps, contract_address)?)
        }
    }
}

pub fn query_swap(deps: Deps<TerraQueryWrapper>, offer_coin: Coin, ask_denom: String) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate(deps: Deps<TerraQueryWrapper>) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap(deps: Deps<TerraQueryWrapper>, denom: String) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxCapResponse = querier.query_tax_cap(denom)?;

    Ok(res)
}

pub fn query_exchange_rates(
    deps: Deps<TerraQueryWrapper>,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;

    Ok(res)
}

pub fn query_contract_info(
    deps: Deps<TerraQueryWrapper>,
    contract_address: String,
) -> StdResult<ContractInfoResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ContractInfoResponse = querier.query_contract_info(contract_address)?;

    Ok(res)
}
