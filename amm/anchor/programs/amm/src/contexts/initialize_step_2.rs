use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::AmmConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeStep2<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"amm_config", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, AmmConfig>,

    mint_x: InterfaceAccount<'info, Mint>,

    mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"amm_mint_lp", config.key().as_ref()],
        bump = config.mint_lp_bump
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeStep2<'info> {
    pub fn complete_init_step_2(&mut self) -> Result<()> {
        if self.config.init_state != 1 {
            panic!(); // TODO
        }
        self.config.init_state = 2;
        Ok(())
    }
}
