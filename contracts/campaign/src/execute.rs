use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, from_binary, Coin};
use cw20::Cw20ReceiveMsg;

use crate::{error::ContractError, state::CONFIG, msg::Cw20HookMsg};

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

pub fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Deposit {
            amount,
            
        }) => execute_deposit(deps, info, amount, cw20_msg),
        _ => Err(ContractError::CustomError {
            val: "Invalid Cw20HookMsg".to_string(),
        }),
    }
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


pub fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
    amount: Coin,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let cw20_contract = info.sender.to_string();
    
    // check if amount is greater than 0


    Ok(Response::new()
                .add_attribute("execute", "deposit")
                .add_attribute("depositor", "purchase")
            )
                
}

pub fn execute_native_deposit(
    deps: DepsMut,
    info: MessageInfo,
    amount: Coin,
) -> Result<Response, ContractError> {
    // check if amount is greater than 0

    // check if campaign expired ?
    // -> return error

    // check if sender already is in donors list
    // -> increment amount : add to donations list



    Ok(Response::default())
}

pub fn execute_redeem(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // check if amount is goal is reached
    // check if campaign expired ?
    // -> : return error

    // check if sender is in donors list
    // -> : unauthorized

    Ok(Response::default())
}
