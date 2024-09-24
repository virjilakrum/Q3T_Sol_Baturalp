use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Config;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = config,
        mint::token_program = token_program,
        seeds = [b"rewards_mint", config.key().as_ref()],
        bump
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,

    token_program: Interface<'info, TokenInterface>,

    system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init_config(
        &mut self,
        points_per_stake: u8,
        max_stakes: u8,
        freeze_min_days: u16,
        bumps: &InitBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            points_per_day: points_per_stake,
            max_stakes,
            freeze_min_days,
            rewards_mint_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
