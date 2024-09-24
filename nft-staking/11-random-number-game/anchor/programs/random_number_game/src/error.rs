use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("Invalid bump.")]
    InvalidBump,

    #[msg("Overflow error.")]
    OverflowError,

    #[msg("Minimum bet amount is 0.01 SOL.")]
    BetAmountTooLow,

    #[msg("Bet amount exceeds maximum.")]
    BetAmountTooHigh,

    #[msg("Minimum roll is 2.")]
    RollTooLow,

    #[msg("Maximum roll is 96.")]
    RollTooHigh,

    #[msg("Timeout not reached yet.")]
    TimeoutNotReached,

    #[msg("Ed25519 header error.")]
    Ed25519HeaderError,

    #[msg("Ed25519 pubkey error.")]
    Ed25519PubkeyError,

    #[msg("Ed25519 message error.")]
    Ed25519MessageError,

    #[msg("Ed25519 signature error.")]
    Ed25519SignatureError,

    #[msg("Ed25519 program error.")]
    Ed25519ProgramError,

    #[msg("Ed25519 accounts error.")]
    Ed25519AccountsError,

    #[msg("Ed25519 data length error.")]
    Ed25519DataLengthError,
}
