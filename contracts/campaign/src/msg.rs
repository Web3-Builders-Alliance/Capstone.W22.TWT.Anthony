use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::{Collected, Config};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub expiration: u64, // timestamp in seconds
    pub goal: Coin,
    pub recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    Redeem {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},

    #[returns(Collected)]
    GetCollected,
}

#[cw_serde]
pub struct MigrateMsg {}
