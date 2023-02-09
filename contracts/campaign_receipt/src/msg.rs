use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw721_base::MintMsg;

#[cw_serde]
pub enum ExecuteMsg<T> {
    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<T>),
    /// Updates metadata of the NFT
    UpdateMetadata {
        token_id: String,
        amount: Uint128,
    },
    Burn {
        token_id: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetTotalInvested { address: String },
}
