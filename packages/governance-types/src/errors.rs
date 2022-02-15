use cosmwasm_std::Addr;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("AlreadyVoted")]
    AlreadyVoted { voter: Addr },

    #[error("AlreadyVoted")]
    NotFulfilled { }, // The specified requirements haven't been met yet.

    #[error("AlreadyVoted")]
    AlreadyEnded {},    
}
