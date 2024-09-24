use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("Name must be between 1 and 32 characters")]
    InvalidNameLength,
}
