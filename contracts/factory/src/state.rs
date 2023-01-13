use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr};
use cw_storage_plus::{Item};

#[cw_serde]
pub struct CodeIds {
    pub pool: u64,
    pub cw3: u64,
    pub cw20: u64,
    pub cw721: u64,
}
#[cw_serde]
pub struct Config {
    pub amdin: Addr,
    pub code_ids: CodeIds,
}

pub const CONFIG: Item<Config> = Item::new("config");

// we want to store campaigns as a map campaign_id -> campaign
// pub const CAMPAIGNS:
