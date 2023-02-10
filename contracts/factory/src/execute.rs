use cosmwasm_std::{
    to_binary, Addr, Coin, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg, WasmMsg,
};

use crate::{
    error::ContractError,
    msg::InstantiateCampaignMsg,
    reply::CREATE_CAMPAIGN_REPLY_ID,
    state::{CodeIds, CONFIG, TEMP_CAMPAIGN_CREATOR, CAMPAIGNS},
};

/*
    - does campaign needs a cw20 token? -> should provide cw20_initMsg
    - should provide cw721_initMsg
    - should provide cw3_mintMsg
*/
pub fn execute_create_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    expiration: u64, // Timestamp in seconds,
    goal: Coin,
    recipient: String,
) -> Result<Response, ContractError> {
    // get campaign code_id
    let config = CONFIG.load(deps.storage)?;
    
    let campaign = CAMPAIGNS.may_load(deps.storage, info.sender.to_string())?;

    // does this make sense, to limit campaign creation to 1 per creator?
    match campaign {
        Some(_) => Err(ContractError::TooManyCampaign {}),
        None => Ok(()),
    }?;

    // temporarily store campaign creator
    TEMP_CAMPAIGN_CREATOR.save(deps.storage, &info.sender.to_string())?;

    // should controls be done there instead of inside the campaign contract?
    // instantiate campaign
    let campaign_init_msg = InstantiateCampaignMsg {
        name,
        expiration,
        goal,
        recipient,
    };

    Ok(Response::new()
        .add_attribute("method", "create_campaign")
        .add_attribute("creator", info.sender)
        .add_submessage(SubMsg {
            // instantiate
            msg: WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: config.code_ids.campaign,
                msg: to_binary(&campaign_init_msg)?,
                funds: vec![],
                label: "create campaign".to_string(),
            }
            .into(),
            gas_limit: None,
            id: CREATE_CAMPAIGN_REPLY_ID,
            reply_on: ReplyOn::Success,
        }))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin: Option<Addr>,
    code_ids: CodeIds,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(admin) = admin {
        let valid = deps.api.addr_validate(admin.as_ref())?;
        config.admin = valid
    }

    config.code_ids = code_ids;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "update_config"))
}
