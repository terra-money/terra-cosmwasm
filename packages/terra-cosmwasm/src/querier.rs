use cosmwasm_std::{Coin, QuerierWrapper, StdResult};

use crate::query::{
    BankTotalResponse, ContractInfoResponse, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, TerraQuery, TerraQueryWrapper,
};
use crate::route::TerraRoute;

/// This is a helper wrapper to easily use our custom queries
pub struct TerraQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> TerraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a>) -> Self {
        TerraQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Market,
            query_data: TerraQuery::Swap {
                offer_coin,
                ask_denom: ask_denom.into(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Treasury,
            query_data: TerraQuery::TaxCap {
                denom: denom.into(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Treasury,
            query_data: TerraQuery::TaxRate {},
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Oracle,
            query_data: TerraQuery::ExchangeRates {
                base_denom: base_denom.into(),
                quote_denoms: quote_denoms.into_iter().map(|x| x.into()).collect(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Wasm,
            query_data: TerraQuery::ContractInfo {
                contract_address: contract_address.into(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_bank_total<T: Into<String>>(&self, denom: T) -> StdResult<BankTotalResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Bank,
            query_data: TerraQuery::Total {
                denom: denom.into(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }
}
