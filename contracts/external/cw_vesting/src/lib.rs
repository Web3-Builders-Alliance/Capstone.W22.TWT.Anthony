
pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

// so that consumers don't need a cw_ownable dependency to consume this contract's queries.
pub use cw_ownable::Ownership;

#[cfg(test)]
mod tests;