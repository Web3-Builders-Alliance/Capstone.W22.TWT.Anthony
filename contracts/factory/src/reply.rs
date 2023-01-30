use cosmwasm_std::{DepsMut, Env, Reply, Response};
use cw_utils::parse_reply_instantiate_data;

use crate::{
    error::ContractError,
    state::{CAMPAIGNS, TEMP_CAMPAIGN_CREATOR},
};

pub const CREATE_CAMPAIGN_REPLY_ID: u64 = 1;

pub fn handle_campaign_creation_reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg).unwrap();

    let creator = TEMP_CAMPAIGN_CREATOR.load(deps.storage)?;

    // TODO: check if creator has already created a campaign -> returns error

    // store campaign contract address
    CAMPAIGNS.save(deps.storage, creator, &res.contract_address)?;

    // remove temp creator
    TEMP_CAMPAIGN_CREATOR.remove(deps.storage);

    Ok(Response::new()
        .add_attribute("action", "instantiated by factory")
        .add_attribute("campaign_addr", res.contract_address))
}
