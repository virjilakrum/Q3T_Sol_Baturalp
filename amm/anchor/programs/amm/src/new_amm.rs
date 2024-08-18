use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::collections::HashMap;
declare_id!("FTyBtbmmdwLvGXUsJSJqRQkkPpL63L5yLjSakJGrvvYZ");

pub mod contexts;
pub use contexts::*;
pub mod state;
use state::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        emergency_owner: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.seed = seed;
        config.fee = fee;
        config.emergency_owner = emergency_owner;
        config.paused = false;
        config.whitelist = HashMap::new();
        Ok(())
    }
