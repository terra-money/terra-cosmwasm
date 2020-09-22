use cosmwasm_std::{
    to_binary, Api, Binary, Coin, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, TerraMsgWrapper, TerraQuerier,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse<TerraMsgWrapper>> {
    match msg {
        HandleMsg::MsgSwap {
            offer_coin,
            ask_denom,
        } => handle_msg_swap(deps, env, offer_coin, ask_denom, None),
        HandleMsg::MsgSwapSend {
            offer_coin,
            ask_denom,
            recipient,
        } => handle_msg_swap(deps, env, offer_coin, ask_denom, Some(recipient)),
    }
}

pub fn handle_msg_swap<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    env: Env,
    offer_coin: Coin,
    ask_denom: String,
    recipient: Option<HumanAddr>,
) -> StdResult<HandleResponse<TerraMsgWrapper>> {
    let msg = if let Some(recipient) = recipient {
        create_swap_send_msg(env.message.sender, recipient, offer_coin, ask_denom)
    } else {
        create_swap_msg(env.message.sender, offer_coin, ask_denom)
    };

    let res = HandleResponse {
        messages: vec![msg],
        log: vec![],
        data: None,
    };
    Ok(res)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
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

pub fn query_swap<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    offer_coin: Coin,
    ask_denom: String,
) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    denom: String,
) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxCapResponse = querier.query_tax_cap(denom)?;

    Ok(res)
}

pub fn query_exchange_rates<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;

    Ok(res)
}
