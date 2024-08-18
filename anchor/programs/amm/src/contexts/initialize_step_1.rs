use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::AmmConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeStep1<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + AmmConfig::INIT_SPACE,
        seeds = [b"amm_config", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, AmmConfig>,

    mint_x: InterfaceAccount<'info, Mint>,

    mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"amm_mint_lp", config.key().as_ref()],
        bump
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    token_program: Interface<'info, TokenInterface>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeStep1<'info> {
    pub fn complete_init_step_1(
        &mut self,
        seed: u64,
        fee: u16,
        config_bump: u8,
        mint_lp_bump: u8,
    ) -> Result<()> {
        self.config.set_inner(AmmConfig {
            init_state: 1,
            seed,
            fee,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            mint_lp_bump,
            bump: config_bump,
        });
        Ok(())
    }
}
