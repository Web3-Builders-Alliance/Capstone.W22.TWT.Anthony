use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Timestamp;
use cw20::Balance;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub expiration: Timestamp,
    pub goal: Balance,
    pub funds_recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterReceiptContract { contract_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},
}

#[cw_serde]
pub struct MigrateMsg {}
