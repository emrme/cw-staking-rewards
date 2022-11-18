use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Invalid amount")]
    InvalidAmount,
}

// impl From<StdError> for ContractError {
//     fn from(err: StdError) -> ContractError {
//         match err {
//             StdError::Overflow { .. } => ContractError::InvalidAmount {},
//             StdError::NotFound { .. } => ContractError::Unauthorized {},
//             _ => ContractError::Unauthorized {},
//         }
//     }
// }
