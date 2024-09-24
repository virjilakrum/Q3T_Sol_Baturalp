use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmmConfig {
    pub init_state: u8,
    pub seed: u64,
    pub fee: u16, // 0% => 0, 100% => 10000
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp_bump: u8,
    pub bump: u8,
}
