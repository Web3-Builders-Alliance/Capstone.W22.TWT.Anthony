#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::try_update_counter;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_counter;
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:reward-pay&ment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ZERO_CODE: i32 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State { counter: 0 };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("counter", ZERO_CODE.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {} => try_update_counter(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Counter {} => query_counter(deps, env),
    }
}

#[cfg(test)]
mod tests {

    use crate::msg::InstantiateMsg;
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::{instantiate, ZERO_CODE};

    const ADDR1: &str = "addr1";

    // instantiate with provide admin address
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {};
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            resp.attributes,
            vec![attr("counter", ZERO_CODE.to_string())]
        )
    }
}
