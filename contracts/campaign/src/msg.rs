use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Coin};
use cw20::{Balance, Cw20ReceiveMsg};

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub expiration: Timestamp,
    pub goal: Balance,
    pub recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // RegisterReceiptContract { contract_addr: String },
    Receive(Cw20ReceiveMsg),
    Deposit{ amount: Coin },
    Redeem{}
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},
}

#[cw_serde]
pub struct MigrateMsg {}


#[cw_serde]
pub enum Cw20HookMsg {
    Deposit {
        amount: Coin
    },
}