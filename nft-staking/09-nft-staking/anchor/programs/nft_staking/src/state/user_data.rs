use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserData {
    pub points: u32,
    pub staked_count: u8,
    pub bump: u8,
}
