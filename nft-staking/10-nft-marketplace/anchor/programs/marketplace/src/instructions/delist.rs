use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        close = maker,
        seeds = [b"listing", marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = nft_mint,
        associated_token::authority = maker
    )]
    maker_ata_for_nft: InterfaceAccount<'info, TokenAccount>,

    associated_token_program: Program<'info, AssociatedToken>,

    token_program: Interface<'info, TokenInterface>,

    system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let listing_seeds = &[
            b"listing".as_ref(),
            &self.marketplace.key().to_bytes()[..],
            &self.nft_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&listing_seeds[..]];
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            to: self.maker_ata_for_nft.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer_checked(ctx, 1, 0)
    }

    // TODO Close the vault account
}
