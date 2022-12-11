use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{state::DUMMY_CONFIG, ContractError};

pub fn execute_dummy(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _param1: String,
    _param2: String,
) -> Result<Response, ContractError> {
    // load state
    let _config = DUMMY_CONFIG.load(deps.storage)?;

    // TODO: do something with params

    Ok(Response::default())
}
