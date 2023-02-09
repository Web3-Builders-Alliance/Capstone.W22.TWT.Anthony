use cosmwasm_std::{DepsMut, Env, Reply, Response};
use cw_utils::parse_reply_instantiate_data;

use crate::{error::ContractError, state::CONFIG};

pub const INSTANTIATE_RECEIPT_REPLY_ID: u64 = 1;
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 2;

pub fn handle_instantiate_receipt_reply(
    _deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg).unwrap();

    // store receipt contract address
    CONFIG.update(_deps.storage, |mut config| -> Result<_, ContractError> {
        config.receipt_contract = res.clone().contract_address;
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute("action", "instantiated by factory")
        .add_attribute("receipt_address", res.contract_address))
}

pub fn handle_instantiate_token_reply(
    _deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg).unwrap();

    // store receipt contract address
    CONFIG.update(_deps.storage, |mut config| -> Result<_, ContractError> {
        config.token_contract = res.clone().contract_address;
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute("action", "instantiated by factory")
        .add_attribute("receipt_address", res.contract_address))
}
