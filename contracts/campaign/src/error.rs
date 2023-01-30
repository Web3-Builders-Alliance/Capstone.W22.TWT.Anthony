use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // #[error("{0}")]
    // ParseReplyError(#[from] ParseReplyError),
    #[error("An unknown reply ID was received.")]
    UnknownReplyID {},

    #[error("Goal cannot be empty")]
    EmptyGoal {},

    #[error("Unsupported coin")]
    UnsupportedCoin {},

    #[error("Invalid Balance")]
    InvalidBalance {},

    #[error("Contract is underfunded")]
    NotEnoughFunds {},
    

    #[error(
        "Invalid expiration date, a date in the past was provided: {0}",
        provided
    )]
    ExpirationInPast { provided: String },

    #[error("Campaign has expired")]
    Expired {},

    #[error("User is not an investor")]
    NothingToRedeem {},
    
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
