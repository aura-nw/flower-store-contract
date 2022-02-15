use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// use cosmwasm_std::Addr;
use cw_storage_plus::{Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Flower {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub price: i32,
}

pub const FLOWER: Map<'a, (&'a id, &'a id), Expiration> = Map::new("flower");
