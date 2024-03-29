use campaign_receipt::contract::{Metadata, Payment};
use campaign_receipt::msg::ExecuteMsg::UpdateMetadata;
use cosmwasm_std::{
    to_binary, Attribute, BankMsg, Coin, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg, WasmQuery,
};
use cw20::Balance;
use cw721::{Cw721QueryMsg, NftInfoResponse, TokensResponse};

use cw_denom::UncheckedDenom;
use cw_vesting::state::UncheckedVestingParams;
use factory::msg::QueryMsg::GetPayrollFactory;

use cw721_base::ExecuteMsg::Mint;
use cw721_base::{MintMsg, QueryMsg};
use cw_utils::must_pay;

use cw_payroll_factory::msg::ExecuteMsg::InstantiateNativePayrollContract;
use wynd_utils::Curve;

use crate::helper::{check_if_expired, check_if_goal_reached};

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

    check_if_expired(deps.storage, env.clone())?;

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
                    total: deposited,
                    payments: vec![Payment {
                        amount: deposited,
                        date: env.block.time.seconds(),
                    }],
                },
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
            .add_attributes(attributes)
            .add_messages(msgs), // might not be executed in the right order ??
    )
}
// TODO implement balance payroll for recipient
// TODO implement cw20 payroll for investors
pub fn execute_redeem(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let storage = deps.storage;
    check_if_expired(storage, env.clone())?;

    let config = CONFIG.load(storage)?;
    let receipt = config.receipt_contract;

    let tokens: TokensResponse = deps
        .querier
        .query_wasm_smart(
            receipt.clone(),
            &Cw721QueryMsg::Tokens {
                owner: _info.sender.to_string(),
                start_after: None,
                limit: None,
            },
        )
        .unwrap_or_else(|_| TokensResponse { tokens: vec![] });

    if tokens.tokens.is_empty() {
        return Err(ContractError::NothingToRedeem {});
    } else {
        let reached = check_if_goal_reached(storage)?;

        if reached {
            // should be able to redeem perks
            // if recipient => init balance vesting
            if _info.sender == config.recipient {
                // init cw_vesting of project token
            } else {
                // if campaign has cw20 => mint user's token => init cw_vesting
                if config.token_contract != "" {
                    // init cw_vesting of user's token

                    // mint cw20 token
                    let cw20_init_msg = WasmMsg::Execute {
                        contract_addr: config.token_contract,
                        msg: to_binary(&cw20_base::msg::ExecuteMsg::Mint {
                            recipient: _info.sender.to_string(),
                            amount: Uint128::from(1000000u128),
                        })?,
                        funds: vec![],
                    };
                }
            }
        } else {
            // check that contracts has enough funds
            let denom = config.goal.denom;
            let contract_balance = deps.querier.query_balance(env.contract.address, &denom)?;
            let msg: QueryMsg<Empty> = QueryMsg::NftInfo {
                token_id: _info.sender.to_string(),
            };
            let query = WasmQuery::Smart {
                contract_addr: receipt,
                msg: to_binary(&msg)?,
            }
            .into();
            let nft_info: NftInfoResponse<Metadata> = deps.querier.query(&query)?;

            // sum up all user payments
            let mut total_invested: Uint128 = Uint128::zero();
            for payment in nft_info.extension.payments {
                total_invested += payment.amount;
            }
            if contract_balance.amount < total_invested {
                return Err(ContractError::NotEnoughFunds {});
            } else {
                // send funds to user
                let send_msg = BankMsg::Send {
                    to_address: _info.sender.to_string(),
                    amount: vec![Coin {
                        denom,
                        amount: total_invested,
                    }],
                };
                return Ok(Response::new().add_message(send_msg));
            }
        }
    }

    Ok(Response::default())
}

pub fn instantiate_vesting(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    balance: Balance,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let payroll_factory = deps
        .querier
        .query_wasm_smart(config.payroll_factory_contract, &GetPayrollFactory {})?;

    match balance {
        Balance::Native(native) => {
            // send NativeInstatiateMsg to payroll factory to initiate native vesting
            let native_init_msg = WasmMsg::Execute {
                contract_addr: payroll_factory,
                msg: to_binary(&InstantiateNativePayrollContract {
                    instantiate_msg: cw_vesting::msg::InstantiateMsg {
                        owner: None,
                        params: UncheckedVestingParams {
                            recipient: info.sender.to_string(),
                            amount: native.0[0].amount,
                            denom: UncheckedDenom::Native(native.0[0].denom.clone()),
                            // check Curve settings -> could make it configurable
                            vesting_schedule: Curve::Constant {
                                y: (Uint128::from(150u64)),
                            },
                            title: Some("vesting".to_string()),
                            description: None,
                        },
                    },
                    label: "vesting".to_string(),
                })?,
                funds: vec![],
            };
        }
        Balance::Cw20(cw20) => {
            // send amount of cw20 to payroll factory to initiate cw20 vesting
            let cw20_send_msg = WasmMsg::Execute {
                contract_addr: cw20.address.to_string(),
                msg: to_binary(&cw20_base::msg::ExecuteMsg::Send {
                    contract: payroll_factory,
                    amount: cw20.amount,
                    msg: to_binary(
                        &cw_payroll_factory::msg::ReceiveMsg::InstantiatePayrollContract {
                            instantiate_msg: cw_vesting::msg::InstantiateMsg {
                                owner: None,
                                params: UncheckedVestingParams {
                                    recipient: info.sender.to_string(),
                                    amount: cw20.amount,
                                    denom: UncheckedDenom::Cw20(cw20.address.to_string()),
                                    // check Curve settings -> could make it configurable
                                    vesting_schedule: Curve::Constant {
                                        y: (Uint128::from(150u64)),
                                    },
                                    title: Some("vesting".to_string()),
                                    description: None,
                                },
                            },
                            label: "vesting".to_string(),
                        },
                    )?,
                })?,
                funds: vec![],
            };
        }
    }
    

    

    Ok(Response::default())
}
