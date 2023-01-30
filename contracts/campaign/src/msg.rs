use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::Config;

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
}

#[cw_serde]
pub struct MigrateMsg {}

// #[cw_serde]
// pub enum Cw20HookMsg {
//     Deposit { balance: Balance, sender: String },
// }

// // Adding default to the struct will allow us to use it in executes
// #[cw_serde]
// #[derive(Default)]
// pub struct TokensResponse {
//     pub tokens: Vec<String>
// }
