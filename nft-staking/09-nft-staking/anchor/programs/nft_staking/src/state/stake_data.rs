use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeData {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64, // Unix timestamp
    pub bump: u8,
}
