use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::EscrowData;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow_data", maker.key().as_ref(), escrow_data.seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow_data: Account<'info, EscrowData>,

    #[account(
        mint::token_program = token_program
    )]
    mint_a: InterfaceAccount<'info, Mint>,

    // Receives Token A's from the escrow
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Sends Token A's to the maker
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_data,
        associated_token::token_program = token_program
    )]
    escrow_ata_a: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        // STEP 1: Send Token A's from the escrow to the maker

        // let binding = self.escrow_data.seed.to_le_bytes();
        // let escrow_data_seeds = &[
        //     b"escrow_data",
        //     self.maker.to_account_info().key.as_ref(),
        //     binding.as_ref(),
        //     &[self.escrow_data.bump],
        // ];
        // let signer_seeds = &[&escrow_data_seeds[..]];
        let seed = self.escrow_data.seed.to_le_bytes();
        let bump = [self.escrow_data.bump];
        let signer_seeds = &[&[
            b"escrow_data",
            self.maker.to_account_info().key.as_ref(),
            seed.as_ref(),
            &bump,
        ][..]];

        let accounts = TransferChecked {
            from: self.escrow_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow_data.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, self.escrow_ata_a.amount, self.mint_a.decimals)?;

        // STEP 3: Close the escrow ATA for mint_a

        let accounts = CloseAccount {
            account: self.escrow_ata_a.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow_data.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        close_account(ctx)
    }
}
