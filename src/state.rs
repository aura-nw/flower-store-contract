use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

use cosmwasm_std::Storage;
// use cw_storage_plus::Map;

static STORE_KEY: &[u8] = b"store";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Flower {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub price: i32,
}

// pub const FLOWER: Map<'a, (&'a String, &'a id), Expiration> = Map::new("flower");

pub fn store(storage: &mut dyn Storage) -> Bucket<Flower> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Flower> {
    bucket_read(storage, STORE_KEY)
}
