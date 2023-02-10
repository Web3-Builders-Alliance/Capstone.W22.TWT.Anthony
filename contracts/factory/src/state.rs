use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct CodeIds {
    pub cw3: u64,
    pub cw20: u64,
    pub campaign: u64,
    pub receipt: u64,
}
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub code_ids: CodeIds,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const TEMP_CAMPAIGN_CREATOR: Item<String> = Item::new("temp_campaign_creator");

// we want to store campaigns as a map creator -> campaigns
pub const CAMPAIGNS: Map<String, String> = Map::new("campaigns");
