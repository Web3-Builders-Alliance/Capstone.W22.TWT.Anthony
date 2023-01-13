use cosmwasm_schema::{cw_serde, QueryResponses};
use cw20::Balance;
use cw_utils::Expiration;

use cw20_base::msg::InstantiateMsg as CW20InstantiateMsg;

pub use crate::state::{CodeIds, Config};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCampaign {
        expiration: Expiration,
        threshold: Balance,
        funds_recipient: String,
        cw20_init_msg: Option<crate::msg::InitMsgEnum>,
        cw721_init_msg: InitMsgEnum,
    },
    UpdateConfig {
        admin: Option<String>,
        code_ids: CodeIds,
    },
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
pub struct PoolInfo {
    pub name: String,
    pub symbol: String,
    pub expiration: Expiration,
    pub init_msg: InitMsgEnum,
}

#[cw_serde]
pub enum InitMsgEnum {
    Cw20InitMsg {
        msg: CW20InstantiateMsg,
    },
    // cw3_initMsg{
    //     voters: Vec<Voter>,
    //     required_weight: u64,
    //     max_voting_period: Option<u64>,
    //     description: String,
    // },
    Cw721InitMsg {
        name: String,
        symbol: String,
        minter: String,
    },
}
