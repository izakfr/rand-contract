use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Storage};
use cosmwasm_storage::{singleton_read, ReadonlySingleton, Singleton, PrefixedStorage, prefixed, prefixed_read, ReadonlyPrefixedStorage};

const CONFIG_KEY: &[u8] = b"config";
const BEACONS_KEY: &[u8] = b"beacons";
const BOUNTIES_KEY: &[u8] = b"bounties";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub pubkey: Binary,
    pub bounty_denom: String,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn beacons_storage(storage: &mut dyn Storage) -> PrefixedStorage {
    prefixed(storage, BEACONS_KEY)
}

pub fn beacons_storage_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    prefixed_read(storage, BEACONS_KEY)
}

pub fn bounties_storage(storage: &mut dyn Storage) -> PrefixedStorage {
    prefixed(storage, BOUNTIES_KEY)
}

pub fn bounties_storage_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    prefixed_read(storage, BOUNTIES_KEY)
}
