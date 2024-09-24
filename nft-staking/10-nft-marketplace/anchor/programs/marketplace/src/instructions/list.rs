use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = maker,
        space = 8 + Listing::INIT_SPACE,
        seeds = [b"listing", marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = nft_metadata.collection.as_ref().unwrap().key.as_ref() == nft_collection.key().as_ref(),
        constraint = nft_metadata.collection.as_ref().unwrap().verified
    )]
    nft_metadata: Account<'info, MetadataAccount>,

    // TODO Is this necessary?
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    nft_edition: Account<'info, MasterEditionAccount>,

    nft_collection: InterfaceAccount<'info, Mint>,

    #[account(
        // TODO Is mut necessary here?
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = maker
    )]
    maker_ata_for_nft: InterfaceAccount<'info, TokenAccount>,

    metadata_program: Program<'info, Metadata>,

    associated_token_program: Program<'info, AssociatedToken>,

    token_program: Interface<'info, TokenInterface>,

    system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            mint: self.nft_mint.key(),
            price,
            bump: bumps.listing,
        });
        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.maker_ata_for_nft.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, 1, 0)
    }
}
