use cosmwasm_std::{ Env, Timestamp, Storage, DepsMut, Querier, QuerierWrapper};
use cw721::{TokensResponse, Cw721QueryMsg};
use cw_utils::Expiration;

use crate::{error::ContractError, state::{Config, CONFIG, COLLECTED_AMOUNT}};


pub fn check_if_expired (storage: &mut dyn Storage, env: Env) -> Result<bool, ContractError> {
    let config: Config = CONFIG.load(storage)?;
    let expiration = Expiration::AtTime(Timestamp::from_seconds(config.expiration));
    if expiration.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }
    Ok(false)
}

pub fn check_if_goal_reached (storage: &mut dyn Storage) -> Result<bool, ContractError> {
    let config: Config = CONFIG.load(storage)?;
    let  collected_amount = COLLECTED_AMOUNT.load(storage)?;

    if collected_amount.coin.amount >= config.goal.amount {
       return Ok(true)
    }
    Ok(false)
}
