use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Timelock duration is invalid")]
    LockDurationInvalid,

    #[msg("Timelock contract not started")]
    LockContractNotStarted,

    #[msg("Timelock contract ended")]
    LockContractEnded,
}
