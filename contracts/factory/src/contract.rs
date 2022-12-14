#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Deps, StdResult, Binary, Reply};
use cw2::set_contract_version;
use crate::{msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg}, error::ContractError};

pub(crate) const CONTRACT_NAME: &str = "crates.io:campaign-factory";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// add cosmwasm smart contract boiler plate
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePool {name, symbol, cw20_code_id, cw3_code_id, cw721_code_id } => execute_create_pool(deps, env, info, name, symbol, cw20_code_id, cw3_code_id, cw721_code_id),
        ExecuteMsg::UpdateConfig { admin, code_ids } => execute_update_config(deps, env, info, admin, code_ids)
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        _ => Err(ContractError::UnknownReplyID {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}