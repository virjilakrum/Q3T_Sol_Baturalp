use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("You have reached the maximum number of stakes.")]
    MaxStakesReached,
    #[msg("You haven't reached the minimum freeze period yet.")]
    FreezePeriodNotOver,
    #[msg("The provided mint address is incorrect.")]
    IncorrectMint,
    #[msg("The provided collection address is incorrect.")]
    IncorrectCollection,
    #[msg("The provided collection is not verified.")]
    CollectionNotVerified,
    #[msg("You have already staked this NFT.")]
    NftAlreadyStaked,
    #[msg("The provided NFT is not part of the allowed collection.")]
    NftNotInAllowedCollection,
    #[msg("The staking period has not ended yet.")]
    StakingPeriodNotEnded,
    #[msg("Insufficient funds to perform this operation.")]
    InsufficientFunds,
    #[msg("The provided account is not authorized to perform this action.")]
    UnauthorizedAccess,
    #[msg("The NFT metadata is invalid or cannot be parsed.")]
    InvalidNftMetadata,
    #[msg("The staking pool is currently full.")]
    StakingPoolFull,
}