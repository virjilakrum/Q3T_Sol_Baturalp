use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::EscrowData;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + EscrowData::INIT_SPACE,
        seeds = [b"escrow_data", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow_data: Account<'info, EscrowData>,

    #[account(
        mint::token_program = token_program
    )]
    mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    mint_b: InterfaceAccount<'info, Mint>,

    // Sends Token A's to the escrow
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Receives Token A's from the maker
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_data,
        associated_token::token_program = token_program
    )]
    escrow_ata_a: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn save_escrow_data(
        &mut self,
        seed: u64,
        receive_amount: u64,
        escrow_data_bump: u8,
    ) -> Result<()> {
        self.escrow_data.set_inner(EscrowData {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount,
            bump: escrow_data_bump,
        });
        Ok(())
    }

    pub fn deposit_to_escrow(&self, amount: u64) -> Result<()> {
        let accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.escrow_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, self.mint_a.decimals)
    }
}
