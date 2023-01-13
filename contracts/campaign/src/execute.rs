use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, state::CONFIG};

/*
    - does campaign needs a cw20 token? -> should provide cw20_initMsg
    - should provide cw721_initMsg
    - should provide cw3_mintMsg
*/
pub fn execute_dummy(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn execute_register_receipt_contract(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract_addr: String,
) -> Result<Response, ContractError> {
    // TODO: check if contract_addr is a valid contract address
    let receipt = deps.api.addr_validate(&contract_addr)?;

    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.receipt_contract = receipt.to_string();
        Ok(config)
    })?;

    Ok(Response::default())
}
