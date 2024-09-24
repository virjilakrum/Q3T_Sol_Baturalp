use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::AmmConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeStep3<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"amm_config", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, AmmConfig>,

    mint_x: Box<InterfaceAccount<'info, Mint>>,

    mint_y: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_program,
        seeds = [b"amm_mint_lp", config.key().as_ref()],
        bump = config.mint_lp_bump
    )]
    mint_lp: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeStep3<'info> {
    pub fn complete_init_step_3(&mut self) -> Result<()> {
        if self.config.init_state != 2 {
            panic!(); // TODO
        }
        self.config.init_state = 3;
        Ok(())
    }
}
