use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw20::Balance;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub name: String,
    pub expiration: Timestamp,
    pub goal: Balance,
    pub funds_recipient: Addr,
    pub receipt_contract: String,
}

#[cw_serde]
pub struct Donations {
    pub address: Addr,
    pub amount: Balance,
}

pub const CONFIG: Item<Config> = Item::new("config");

//  store donations as a map user_addr -> coin
pub const DONATIONS: Map<Addr, Coin> = Map::new("donations");
