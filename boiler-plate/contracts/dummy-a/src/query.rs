use cosmwasm_std::{Deps, StdResult};

use crate::{msg::DummyResponse, state::DUMMY_CONFIG};

pub fn query_config(deps: Deps) -> StdResult<DummyResponse> {
    let config = DUMMY_CONFIG.load(deps.storage)?;
    Ok(DummyResponse {
        owner: config.owner.into(),
    })
}
