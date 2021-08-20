use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// TerraRoute is enum type to represent terra query route path
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerraRoute {
    Market,
    Treasury,
    Oracle,
    Wasm,
}
