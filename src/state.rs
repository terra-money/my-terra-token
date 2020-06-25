use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Uint128};
use cosmwasm_storage::{Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static BALANCE_KEY: &[u8] = b"balance";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
    pub symbol: String,
    pub owner: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Config> {
    Singleton::new(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY)
}

pub fn balance_set<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    amount: &Uint128,
) -> StdResult<()> {
    Bucket::new(BALANCE_KEY, storage).save(address.as_slice(), amount)
}

pub fn balance_of<S: Storage>(storage: &S, address: &CanonicalAddr) -> Uint128 {
    match ReadonlyBucket::new(BALANCE_KEY, storage).may_load(address.as_slice()) {
        Ok(Some(amount)) => amount,
        _ => Uint128::zero(),
    }
}
