use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw20::Balance;
use cw_utils::Expiration;

use crate::{error::ContractError, msg::InitMsgEnum, state::CodeIds};

/*
    - does campaign needs a cw20 token? -> should provide cw20_initMsg
    - should provide cw721_initMsg
    - should provide cw3_mintMsg
*/
pub fn execute_create_campaign(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _expiration: Expiration,
    _threshold: Balance,
    _funds_recipient: String,
    _cw20_init_msg: Option<crate::msg::InitMsgEnum>,
    _cw721_init_msg: InitMsgEnum,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn execute_update_config(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _admin: Option<String>,
    _code_ids: CodeIds,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
