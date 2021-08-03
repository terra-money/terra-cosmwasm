mod msg;
mod querier;
mod query;
mod route;

pub use msg::{create_swap_msg, create_swap_send_msg, TerraMsg, TerraMsgWrapper};
pub use querier::TerraQuerier;
pub use query::{
    ContractInfoResponse, ExchangeRateItem, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, TerraQuery, TerraQueryWrapper,
};
pub use route::TerraRoute;

// This export is added to all contracts that import this package, signifying that they require
// "terra" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_terra() {}
