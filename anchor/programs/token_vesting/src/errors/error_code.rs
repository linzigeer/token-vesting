use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Claim time not reached yet!")]
    ClaimTimeNotReachedYet,

    #[msg("Invalid vest period!")]
    InvalidVestPeriod,

    #[msg("Invalid total allocated amount!")]
    InvalidAllocatedAmount,

    #[msg("Invalid cliff time!")]
    InvalidCliffTime,
    
    #[msg("Claim not available now!")]
    ClaimNotAvailableNow,
}