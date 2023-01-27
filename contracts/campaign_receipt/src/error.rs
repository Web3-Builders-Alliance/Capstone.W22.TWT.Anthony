use cosmwasm_std::StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid Address")]
    InvalidAdmin {},

    // #[error("{0}")]
    // ParseReplyError(#[from] ParseReplyError),
    #[error("An unknown reply ID was received.")]
    UnknownReplyID {},
}
