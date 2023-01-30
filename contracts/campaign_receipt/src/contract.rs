use cosmwasm_schema::cw_serde;
use cosmwasm_schema::serde::Deserialize;
use cosmwasm_std::Timestamp;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;

use cosmwasm_std::Empty;
use cw2::set_contract_version;

use cw721_base::Cw721Contract;
use cw721_base::InstantiateMsg;
use serde::Serialize;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:donation- campaign-receipt";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Deserialize, Serialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    // pub name: String,
    pub payments: Vec<Payment>,
    // pub tier: Option<String>,
}

#[cw_serde]
pub struct Payment {
    pub amount: Uint128,
    pub date: Timestamp,
}

impl Default for Payment {
    fn default() -> Self {
        Self {
            amount: Uint128::default(),
            date: Timestamp::from_seconds(0),
        }
    }
}

pub type Extension = Option<Metadata>;

pub type Cw721MetadataNonTransferableContract<'a> =
    Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use crate::msg::ExecuteMsg;
    use cosmwasm_std::{entry_point, to_binary, WasmQuery};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw721::NftInfoResponse;
    use cw721_base::ContractError;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let res = Cw721MetadataNonTransferableContract::default().instantiate(
            deps.branch(),
            env,
            info,
            msg,
        )?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint(mint_msg) => {
                Cw721MetadataNonTransferableContract::default().mint(deps, env, info, mint_msg)
            }
            ExecuteMsg::UpdateMetadata { token_id, amount } => {
                execute_update_on_chain_metadata(deps, env, info, token_id, amount)
            }
            ExecuteMsg::Burn { token_id } => execute_burn(deps, env, info, token_id),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataNonTransferableContract::default().query(deps, env, msg)
    }

    fn execute_update_on_chain_metadata(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let tract = Cw721MetadataNonTransferableContract::default();
        let minter = tract.minter.load(deps.storage)?;
        if info.sender != minter {
            Err(ContractError::Unauthorized {})
        } else {
            // unable to use helper equivalent here .??
            let msg: QueryMsg = QueryMsg::NftInfo {
                token_id: token_id.clone(),
            };
            let query = WasmQuery::Smart {
                contract_addr: _env.contract.address.to_string(),
                msg: to_binary(&msg)?,
            }
            .into();

            // query metadata
            let mut nft_info: NftInfoResponse<Metadata> = deps.querier.query(&query)?;
            nft_info.extension.payments.push(Payment {
                amount,
                date: _env.block.time,
            });

            tract
                .tokens
                .update(deps.storage, &token_id, |token| match token {
                    Some(mut token_info) => {
                        token_info.extension = Some(Metadata {
                            payments: nft_info.extension.payments,
                        });
                        Ok(token_info)
                    }
                    None => Err(ContractError::Unauthorized {}),
                })?;
            Ok(Response::new())
        }
    }
    fn execute_burn(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _token_id: String,
    ) -> Result<Response, ContractError> {
        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;
    use cw721_base::MintMsg;

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataNonTransferableContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                ..Metadata::default()
            }),
        };
        let exec_msg = Cw721ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
