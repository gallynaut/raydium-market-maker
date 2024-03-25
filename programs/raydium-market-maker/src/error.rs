//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the TokenAmm program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RaydiumMarketMakerError {
    // 0
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
}

impl From<RaydiumMarketMakerError> for ProgramError {
    fn from(e: RaydiumMarketMakerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for RaydiumMarketMakerError {
    fn type_of() -> &'static str {
        "Amm Error"
    }
}

impl PrintProgramError for RaydiumMarketMakerError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            RaydiumMarketMakerError::AlreadyInUse => msg!("Error: AlreadyInUse"),
        }
    }
}
