#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, DepsMut, Response};

use crate::error::ContractError;
use crate::msg::ExecuteResponse;
use crate::state::{State, STATE};

pub fn try_update_counter(deps: DepsMut) -> Result<Response, ContractError> {
    let current_state = STATE.load(deps.storage)?;
    let mut current_counter = current_state.counter;

    current_counter += 1;

    let new_state = State {
        counter: current_counter,
    };
    STATE.save(deps.storage, &new_state)?;

    let resp = to_binary(&ExecuteResponse {
        counter: current_counter,
    })
    .unwrap();
    Ok(Response::new().set_data(resp))
}

#[cfg(test)]
mod tests {
    use crate::contract::instantiate;
    use crate::msg::{ExecuteMsg, ExecuteResponse};
    use crate::state::STATE;

    use crate::contract::execute;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::to_binary;

    const ADDR: &str = "addr";

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR, &[]);
        let expect_data = to_binary(&ExecuteResponse { counter: 1 }).unwrap();
        let expect_number = 2;

        // instantiate msg
        let msg = InstantiateMsg {};
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // execute one time
        let msg = ExecuteMsg::Update {};
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        println!("Execute once!");
        assert_eq!(resp.data, Some(expect_data));

        // execute two time
        let msg = ExecuteMsg::Update {};
        let _resp = execute(deps.as_mut(), env, info, msg);
        let current_state = STATE.load(deps.as_mut().storage).unwrap();
        println!("Execute twice!");
        assert_eq!(current_state.counter, expect_number);
    }
}
