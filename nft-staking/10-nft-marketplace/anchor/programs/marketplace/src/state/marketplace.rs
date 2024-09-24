use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub name: String,
    pub fee: u16,
    // pub rewards_mint_bump: u8,
    pub treasury_bump: u8,
    pub bump: u8,
}

impl Space for Marketplace {
    // Here we assume a max length of 32 for `name`
    const INIT_SPACE: usize = 32 + (32 + 4) + 2 + 1 + 1 + 1;
}
