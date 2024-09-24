use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub points_per_day: u8,
    pub max_stakes: u8,
    pub freeze_min_days: u16,
    pub rewards_mint_bump: u8,
    pub bump: u8,
}
