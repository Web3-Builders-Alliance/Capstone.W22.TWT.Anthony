use crate::{
    error::ContractError,
    execute::{execute_register_receipt_contract, execute_deposit, execute_native_deposit, receive_cw20, execute_redeem},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    reply::{handle_instantiate_reply, INSTANTIATE_REPLY_ID},
    state::{Config, CONFIG},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::Expiration;

use factory::msg::{Config as FactoryConfig, InitMsgEnum, QueryMsg::GetConfig as GetFactoryConfig};

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
    if Expiration::AtTime(msg.expiration).is_expired(&_env.block) {
        return Err(ContractError::ExpirationInPast {
            provided: msg.expiration.to_string(),
        });
    }

    // shall we check that expiration is not too far in the future? -> define a campaign max duration

    // check that goal is not 0
    if msg.goal.is_empty() {
        return Err(ContractError::EmptyGoal {});
    }

    let cloned_msg = msg.clone();

    // check that funds recipient is a valid address
    let address_to = deps.api.addr_validate(&msg.recipient)?;

    CONFIG.save(
        deps.storage,
        &Config {
            name: msg.name,
            expiration: msg.expiration,
            goal: msg.goal,
            recipient: address_to,
            receipt_contract: "".to_string(), // will be set in the handle_instantiate_reply function
            factory_contract: Addr::from(info.sender.clone()),
        },
    )?;

    // get code_ids from factory
    let factory_config: FactoryConfig = deps
        .querier
        .query_wasm_smart("factory", &GetFactoryConfig {})
        .unwrap();

    let cw721_init_msg = cw721_base::InstantiateMsg {
        name: cloned_msg.name.to_string(),
        symbol: "campaign_receipt".to_string(),
        minter: _env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender)
        .add_attribute("campaign_name", cloned_msg.name)
        .add_attribute("campaign_goal", cloned_msg.goal.to_string())
        .add_attribute("expiration", msg.expiration.to_string())
        .add_submessage(SubMsg {
            // instantiate
            msg: WasmMsg::Instantiate {
                admin: Some(_env.contract.address.to_string()),
                code_id: factory_config.code_ids.cw721,
                msg: to_binary(&cw721_init_msg)?,
                funds: vec![],
                label: "instantiate campaign receipt".to_string(),
            }
            .into(),
            gas_limit: None,
            id: INSTANTIATE_REPLY_ID,
            reply_on: ReplyOn::Success,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::Receive(cw20_msg) => receive_cw20(deps, env, info, cw20_msg),
    ExecuteMsg::Deposit { amount  } => execute_native_deposit(deps, info, amount),
    ExecuteMsg::Redeem {  } => execute_redeem(deps,info),
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
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, _env, msg),
        _ => Err(ContractError::UnknownReplyID {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
