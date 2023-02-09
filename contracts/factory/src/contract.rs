use crate::{
    error::ContractError,
    execute::{execute_create_campaign, execute_update_config},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    reply::{handle_campaign_creation_reply, CREATE_CAMPAIGN_REPLY_ID},
    state::{Config, CONFIG}, query::query_campaigns,
};
use cosmwasm_schema::serde::Serialize;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

pub(crate) const CONTRACT_NAME: &str = "crates.io:campaign-factory";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = match msg.admin {
        Some(admin) => deps.api.addr_validate(&admin).unwrap(),
        None => info.sender,
    };

    let config = Config {
        admin,
        code_ids: msg.code_ids,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", config.admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateCampaign {
            name,
            expiration,
            goal,
            recipient,
        } => execute_create_campaign(deps, _env, _info, name, expiration, goal, recipient),
        ExecuteMsg::UpdateConfig { admin, code_ids } => {
            execute_update_config(deps, _env, _info, admin, code_ids)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(_deps.storage)?),
        QueryMsg::GetCampaigns { start_after, limit } => to_binary(&query_campaigns(_deps, start_after, limit)?)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        CREATE_CAMPAIGN_REPLY_ID => handle_campaign_creation_reply(_deps, _env, msg),
        _ => Err(ContractError::UnknownReplyID {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

