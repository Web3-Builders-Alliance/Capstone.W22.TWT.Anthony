use cosmwasm_std::{DepsMut, Env, Reply, Response};
use cw_utils::parse_reply_instantiate_data;

use crate::{error::ContractError, state::CONFIG};

pub const INSTANTIATE_REPLY_ID: u64 = 1;

pub fn handle_instantiate_reply(
    _deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg).unwrap();

    // store cw721 contract address
    CONFIG.update(_deps.storage, |mut config| -> Result<_, ContractError> {
        config.receipt_contract = res.clone().contract_address;
        Ok(config)
    })?;


    Ok(Response::new()
        .add_attribute("action", "instantiated by factory")
        .add_attribute("pool_addr", res.contract_address.to_string()))
}
