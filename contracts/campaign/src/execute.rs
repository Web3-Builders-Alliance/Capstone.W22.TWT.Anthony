use campaign_receipt::contract::{Metadata, Payment};
use campaign_receipt::msg::ExecuteMsg::UpdateMetadata;
use cosmwasm_std::{
    to_binary, Attribute, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Timestamp, WasmMsg,
};
use cw721::{Cw721QueryMsg, TokensResponse};
// use cw721_base::MintMsg;
use cw721_base::ExecuteMsg::Mint;
use cw721_base::MintMsg;
use cw_utils::{must_pay, Expiration};

use crate::{
    error::ContractError,
    state::{Collected, COLLECTED_AMOUNT, CONFIG},
};

/*
    - does campaign needs a cw20 token? -> should provide cw20_initMsg
    - should provide cw721_initMsg
    - should provide cw3_mintMsg
*/

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check that only 1 coin is provided of denom config.goal.denom
    let deposited = must_pay(&info, &config.goal.denom).unwrap();

    let receipt = config.receipt_contract;

    // check if campaign expired ?
    if Expiration::AtTime(Timestamp::from_seconds(config.expiration)).is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    // check if sender already is in donors list
    let tokens: TokensResponse = deps
        .querier
        .query_wasm_smart(
            receipt.clone(),
            &Cw721QueryMsg::Tokens {
                owner: info.sender.to_string(),
                start_after: None,
                limit: None,
            },
        )
        .unwrap_or_else(|_| TokensResponse { tokens: vec![] });

    let mut msgs: Vec<WasmMsg> = vec![];
    let mut attributes: Vec<Attribute> = vec![];

    if tokens.tokens.is_empty() {
        // -> mint receipt
        let mint_receipt_msg = WasmMsg::Execute {
            contract_addr: receipt,
            msg: to_binary(&Mint::<Metadata, Empty>(MintMsg {
                token_id: info.sender.to_string(),
                owner: info.sender.to_string(),
                token_uri: None,
                extension: Metadata {
                    payments: vec![Payment{amount: deposited, date: env.block.time}],
                }
            }))?,
            funds: vec![],
        };

        msgs.push(mint_receipt_msg);
        attributes.push(Attribute {
            key: "action".to_string(),
            value: "donating".to_string(),
        });
    } else {
        // -> increment receipt amount
        let increment_receipt_msg = WasmMsg::Execute {
            contract_addr: receipt,
            msg: to_binary(&UpdateMetadata::<Metadata> {
                token_id: info.sender.to_string(),
                amount: deposited,
            })?,
            funds: vec![],
        };
        msgs.push(increment_receipt_msg);
        attributes.push(Attribute {
            key: "action".to_string(),
            value: "top-up".to_string(),
        });
    }

    // -> increment global collected amount
    COLLECTED_AMOUNT.update(deps.storage, |collected| -> StdResult<Collected> {
        collected.coin.amount.checked_add(deposited)?;
        Ok(collected)
    })?;

    Ok(
        Response::new()
            .add_attribute("execute", "deposit")
            .add_attribute("depositor", "purchase")
            .add_attributes(attributes)
            .add_messages(msgs), // might not be executed in the right order ??
    )
}

pub fn execute_redeem(_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    // check if amount is goal is reached
    // check if campaign expired ?
    // -> : return error

    // check if sender is in donors list
    // -> : unauthorized

    Ok(Response::default())
}
