use cosmwasm_schema::cw_serde;
use cw721_base::MintMsg;

use crate::contract::Metadata;

#[cw_serde]
pub enum ExecuteMsg<T> {
    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<T>),
    /// Updates metadata of the NFT
    UpdateMetadata {
        token_id: String,
        metadata: Metadata,
    },
    Burn {
        token_id: String,
    },
}
