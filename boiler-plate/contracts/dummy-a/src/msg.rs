use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    DummyExecute { param1: String, param2: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DummyResponse)]
    DummyQuery {},
}

#[cw_serde]
pub struct DummyResponse {
    pub owner: String,
}
