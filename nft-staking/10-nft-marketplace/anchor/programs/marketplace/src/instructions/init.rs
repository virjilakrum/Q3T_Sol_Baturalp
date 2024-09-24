use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::CustomErrorCode, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Init<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,

    // #[account(
    //     init,
    //     payer = admin,
    //     mint::decimals = 6,
    //     mint::authority = marketplace,
    //     mint::token_program = token_program,
    //     seeds = [b"rewards_mint", marketplace.key().as_ref()],
    //     bump
    // )]
    // rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,

    token_program: Interface<'info, TokenInterface>,

    system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitBumps) -> Result<()> {
        require!(
            name.len() > 0 && name.len() <= 32,
            CustomErrorCode::InvalidNameLength
        );

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            name,
            fee,
            // rewards_mint_bump: bumps.rewards_mint,
            treasury_bump: bumps.treasury,
            bump: bumps.marketplace,
        });
        Ok(())
    }
}
