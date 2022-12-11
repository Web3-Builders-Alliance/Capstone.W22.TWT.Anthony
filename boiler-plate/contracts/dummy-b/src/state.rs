use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct DummyConfig {
    pub owner: String,
}

pub const DUMMY_CONFIG: Item<DummyConfig> = Item::new("config");
