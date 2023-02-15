use cosmwasm_std::{Addr, DepsMut, Env, Reply, Response};
use cw_utils::parse_reply_instantiate_data;

use crate::{
    error::ContractError,
    state::{CAMPAIGNS, PAYROLL_FACTORY, TEMP_CAMPAIGN_CREATOR},
};

pub const CREATE_CAMPAIGN_REPLY_ID: u64 = 1;
pub const INIT_PAYROLL_FACTORY_REPLY_ID: u64 = 2;

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

pub fn handle_payroll_factory_init_reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg).unwrap();

    // store campaign contract address
    PAYROLL_FACTORY.save(deps.storage, &Addr::unchecked(&res.contract_address))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate payroll factory")
        .add_attribute("address", res.contract_address))
}
