use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
    token_interface::{mint_to, Mint, MintTo},
};

use crate::state::{Config, UserData};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump // TODO Does this make sense?
    )]
    config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"user_data", user.key().as_ref()],
        bump = user_data.bump // TODO Does this make sense?
    )]
    user_data: Account<'info, UserData>,

    #[account(
        // TODO Are these necessary?
        // mut,
        // mint::authority = config,
        // mint::token_program = token_program,
        seeds = [b"rewards_mint", config.key().as_ref()],
        bump = config.rewards_mint_bump
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    user_ata_for_reward: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        // STEP 1: Mint reward tokens to the user's ATA
        let amount = self.user_data.points as u64 * 10u64.pow(self.rewards_mint.decimals.into());
        let config_seeds = &[b"config".as_ref(), &[self.config.bump]];
        let signer_seeds = &[&config_seeds[..]];
        let accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.user_ata_for_reward.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        mint_to(ctx, amount)?;

        // STEP 2: Reset user's points
        self.user_data.points = 0;

        Ok(())
    }
}
