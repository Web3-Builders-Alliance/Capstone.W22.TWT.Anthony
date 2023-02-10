use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use cw_utils::{ParseReplyError, PaymentError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownable(#[from] OwnershipError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("instantiate message owner does not match factory owner. got ({actual:?}) expected ({expected:?})")]
    OwnerMissmatch {
        actual: Option<String>,
        expected: Option<String>,
    },

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("Got a submessage reply with unknown id: {id}")]
    UnknownReplyId { id: u64 },
}
