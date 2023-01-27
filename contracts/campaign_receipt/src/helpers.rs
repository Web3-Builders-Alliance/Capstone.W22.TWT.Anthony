use cosmwasm_schema::cw_serde;

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Empty, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};

//use crate::msg::{ExecuteMsg, GetCountResponse, QueryMsg};

pub use cw721::{OwnerOfResponse, TokensResponse};
pub use cw721_base::QueryMsg;

use crate::contract::Cw721ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct NftContract(pub Addr);

impl NftContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<Cw721ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Get Owner of an NFT
    pub fn get_owner<Q, T, CQ>(&self, querier: &Q, token_id: String) -> StdResult<OwnerOfResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg: QueryMsg<Empty> = QueryMsg::OwnerOf {
            token_id,
            include_expired: None,
        };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: OwnerOfResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    /// Get All Tokens
    pub fn all_tokens<Q, T, CQ>(&self, querier: &Q) -> StdResult<TokensResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg: QueryMsg<Empty> = QueryMsg::AllTokens {
            start_after: None,
            limit: None,
        };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: TokensResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
