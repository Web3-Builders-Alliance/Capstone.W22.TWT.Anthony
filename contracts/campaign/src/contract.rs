use crate::{
    error::ContractError,
    execute::{execute_deposit, execute_redeem},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    reply::{ INSTANTIATE_RECEIPT_REPLY_ID, INSTANTIATE_TOKEN_REPLY_ID, handle_instantiate_receipt_reply, handle_instantiate_token_reply},
    state::{Collected, Config, COLLECTED_AMOUNT, CONFIG},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult,
    SubMsg, Timestamp, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::Expiration;

use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;

use factory::msg::{Config as FactoryConfig, QueryMsg::GetConfig as GetFactoryConfig};

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
    // msg.expiration should be given in nanoseconds
    if Expiration::AtTime(Timestamp::from_seconds(msg.expiration)).is_expired(&_env.block) {
        return Err(ContractError::ExpirationInPast {
            provided: msg.expiration.to_string(),
        });
    }

    // shall we check that expiration is not too far in the future? -> define a campaign max duration

    // check that goal is not 0
    if msg.goal.amount.is_zero() {
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
            goal: msg.goal.clone(),
            recipient: address_to,
            receipt_contract: "".to_string(), // will be set in the handle_instantiate_reply function
            factory_contract: info.sender.clone(),
            token_contract: "".to_string(), // will be set in the handle_instantiate_reply function
        },
    )?;

    COLLECTED_AMOUNT.save(deps.storage, &Collected { coin: msg.goal })?;

    // get code_ids from factory
    let factory_config: FactoryConfig = deps
        .querier
        .query_wasm_smart(info.sender.clone(), &GetFactoryConfig {})
        .unwrap();

    let cw721_init_msg = cw721_base::InstantiateMsg {
        name: cloned_msg.name.to_string(),
        symbol: "campaign_receipt".to_string(),
        minter: _env.contract.address.to_string(),
    };

    let cw20_init_msg = Cw20InstantiateMsg {
        name: cloned_msg.name.to_string(),
        symbol: "PRJCT-SRC".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(cw20::MinterResponse {
            minter: _env.contract.address.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender)
        .add_attribute("campaign_name", cloned_msg.name)
        .add_attribute("campaign_goal", cloned_msg.goal.to_string())
        .add_attribute("expiration", msg.expiration.to_string())
        .add_submessage(SubMsg {
            // instantiate receipt
            msg: WasmMsg::Instantiate {
                admin: Some(_env.contract.address.to_string()),
                code_id: factory_config.code_ids.receipt,
                msg: to_binary(&cw721_init_msg)?,
                funds: vec![],
                label: "campaign receipt".to_string(),
            }
            .into(),
            gas_limit: None,
            id: INSTANTIATE_RECEIPT_REPLY_ID,
            reply_on: ReplyOn::Success,
        }).add_submessage(SubMsg {
            // instantiate token
            msg: WasmMsg::Instantiate {
                admin: Some(_env.contract.address.to_string()),
                code_id: factory_config.code_ids.cw20,
                msg: to_binary(&cw20_init_msg)?,
                funds: vec![],
                label: "campaign token".to_string(),
            }
            .into(),
            gas_limit: None,
            id: INSTANTIATE_TOKEN_REPLY_ID,
            reply_on: ReplyOn::Success,
        })
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, env, info),
        ExecuteMsg::Redeem {} => execute_redeem(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(_deps.storage)?),
        QueryMsg::GetCollected {} => to_binary(&COLLECTED_AMOUNT.load(_deps.storage)?),
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_RECEIPT_REPLY_ID => handle_instantiate_receipt_reply(deps, _env, msg),
        INSTANTIATE_TOKEN_REPLY_ID => handle_instantiate_token_reply(deps, _env, msg),
        _ => Err(ContractError::UnknownReplyID {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
