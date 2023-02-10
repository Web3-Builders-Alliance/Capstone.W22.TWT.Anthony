use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub name: String,
    pub expiration: u64, // timestamp in seconds
    pub goal: Coin,
    pub recipient: Addr,
    pub payroll_factory_code_id: u64,
    pub vesting_code_id: u64,
    pub receipt_contract: String,
    pub factory_contract: Addr,
    pub token_contract: String,
    pub payroll_factory_contract: String,
}
#[cw_serde]
pub struct Collected {
    pub coin: Coin,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const COLLECTED_AMOUNT: Item<Collected> = Item::new("collected");
