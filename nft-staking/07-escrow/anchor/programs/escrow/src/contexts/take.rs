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
pub struct Take<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    #[account(mut)]
    maker: SystemAccount<'info>,

    #[account(
        mut,
        close = taker, // Send the rent to the taker (design choice)
        seeds = [b"escrow_data", maker.key().as_ref(), escrow_data.seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow_data: Account<'info, EscrowData>,

    #[account(
        mint::token_program = token_program
    )]
    mint_a: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_program
    )]
    mint_b: Box<InterfaceAccount<'info, Mint>>,

    // Receives Token A's from the escrow
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    // Sends Token B's to the maker
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
        // constraint = taker_ata_b.amount >= escrow_data.receive_amount @ EscrowError::InsufficientBalance
    )]
    taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    // Receives Token B's from the taker
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    // Sends Token A's to the taker
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_data,
        associated_token::token_program = token_program
    )]
    escrow_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn pay_the_maker(&self) -> Result<()> {
        // STEP 1: Send Token B's from the taker to the maker
        let accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, self.escrow_data.receive_amount, self.mint_b.decimals)
    }

    pub fn withdraw_and_close(&mut self) -> Result<()> {
        // STEP 2: Send Token A's from the escrow to the taker

        let tmp_escrow_data_seed = self.escrow_data.seed.to_le_bytes();
        let escrow_data_seeds = &[
            b"escrow_data",
            self.maker.to_account_info().key.as_ref(),
            tmp_escrow_data_seed.as_ref(),
            &[self.escrow_data.bump],
        ];
        let signer_seeds = &[&escrow_data_seeds[..]];

        let accounts = TransferChecked {
            from: self.escrow_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
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
            destination: self.taker.to_account_info(), // Send the rent to the taker (design choice)
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
