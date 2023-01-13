use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    state::{Config, CONFIG}, execute::execute_register_receipt_contract,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, to_binary};
use cw2::set_contract_version;
use cw_utils::Expiration;

pub(crate) const CONTRACT_NAME: &str = "crates.io:campaign";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // check that expiration is not expired
    if Expiration::AtTime(msg.expiration).is_expired(&_env.block){
        return Err(ContractError::ExpirationInPast {provided: msg.expiration.to_string()});
    }

    // shall we check that expiration is not too far in the future? -> define a campaign max duration
    

    // check that goal is not 0
    if msg.goal.is_empty() {
        return Err(ContractError::EmptyGoal {});
    }

    let cloned_msg = msg.clone();

    // check that funds recipient is a valid address
    let address_to = deps.api.addr_validate(&msg.funds_recipient)?;

    CONFIG.save(
        deps.storage,
        &Config {
            name: msg.name,
            expiration: msg.expiration,
            goal: msg.goal,
            funds_recipient: address_to,
            receipt_contract:"".to_string()
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender)
        .add_attribute("campaign_name", cloned_msg.name)
        .add_attribute("campaign_goal", cloned_msg.goal.to_string())
        .add_attribute("expiration", msg.expiration.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::RegisterReceiptContract { contract_addr } => execute_register_receipt_contract(
        _deps,
        _env,
        _info,
        contract_addr,
    ),
}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(_deps.storage)?),
    }
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
