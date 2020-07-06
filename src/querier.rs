use cosmwasm_std::{Coin, Decimal, Querier, StdResult, Uint128};

use crate::query::{SwapResponse, TaxCapResponse, TaxRateResponse, TerraQuery, TerraQueryWrapper};

/// This is a helper wrapper to easily use our custom queries
pub struct TerraQuerier<'a, Q: Querier> {
    querier: &'a Q,
}

impl<'a, Q: Querier> TerraQuerier<'a, Q> {
    pub fn new(querier: &'a Q) -> Self {
        TerraQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(&self, offer_coin: Coin, ask_denom: T) -> StdResult<Coin> {
        let request = TerraQueryWrapper {
            route: "market".to_string(),
            query_data: TerraQuery::Swap {
                offer_coin,
                ask_denom: ask_denom.into(),
            },
        };
        let res: SwapResponse = self.querier.custom_query(&request.into())?;
        Ok(res.receive)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<Uint128> {
        let request = TerraQueryWrapper {
            route: "treasury".to_string(),
            query_data: TerraQuery::TaxCap {
                denom: denom.into(),
            },
        };
        let res: TaxCapResponse = self.querier.custom_query(&request.into())?;
        Ok(res.cap)
    }

    pub fn query_tax_rate(&self) -> StdResult<Decimal> {
        let request = TerraQueryWrapper {
            route: "treasury".to_string(),
            query_data: TerraQuery::TaxRate {},
        };
        let res: TaxRateResponse = self.querier.custom_query(&request.into())?;
        Ok(res.rate)
    }
}
