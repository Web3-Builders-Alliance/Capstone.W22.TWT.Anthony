#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::{
    execute::execute_dummy,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::query_config,
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = "dummy-a";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// add entry point macro
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

// check if sender is admin


    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DummyExecute { param1, param2 } => {
            execute_dummy(deps, env, info, param1, param2)
        }
    }
}



//  add query entry point
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DummyQuery {} => to_binary(&query_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    Ok(Response::default())
}