use cosmwasm_std::{Coin, Querier, StdResult};

use crate::query::{
    ExchangeRatesResponse, SwapResponse, TaxCapResponse, TaxRateResponse, TerraQuery,
    TerraQueryWrapper, TerraRoute,
};

/// This is a helper wrapper to easily use our custom queries
pub struct TerraQuerier<'a, Q: Querier> {
    querier: &'a Q,
}

impl<'a, Q: Querier> TerraQuerier<'a, Q> {
    pub fn new(querier: &'a Q) -> Self {
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
        };
        let res: SwapResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Treasury,
            query_data: TerraQuery::TaxCap {
                denom: denom.into(),
            },
        };
        let res: TaxCapResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = TerraQueryWrapper {
            route: TerraRoute::Treasury,
            query_data: TerraQuery::TaxRate {},
        };
        let res: TaxRateResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
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
        };

        let res: ExchangeRatesResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }
}
