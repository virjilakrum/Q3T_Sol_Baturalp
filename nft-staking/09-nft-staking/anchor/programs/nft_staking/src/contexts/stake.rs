use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::errors::CustomErrorCode;
use crate::state::{Config, StakeData, UserData};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump
    )]
    config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"user_data", user.key().as_ref()],
        bump = user_data.bump // TODO Does this make sense?
    )]
    user_data: Account<'info, UserData>,

    #[account(
        init,
        payer = user,
        space = 8 + StakeData::INIT_SPACE,
        seeds = [b"stake_data", config.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    stake_data: Account<'info, StakeData>,

    nft_mint: Account<'info, Mint>,

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

    nft_collection: Account<'info, Mint>,

    #[account(
        // TODO Is mut necessary here?
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    user_ata_for_nft: Account<'info, TokenAccount>,

    metadata_program: Program<'info, Metadata>,

    token_program: Program<'info, Token>,

    system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        // STEP 0: Check staked count
        require!(
            self.user_data.staked_count < self.config.max_stakes,
            CustomErrorCode::MaxStakesReached
        );

        // STEP 1: Delegate
        let accounts = Approve {
            to: self.user_ata_for_nft.to_account_info(), // The account that holds the tokens
            delegate: self.stake_data.to_account_info(), // The account that can use the tokens (under permission of the owner)
            authority: self.user.to_account_info(), // The account that can use the tokens (the owner)
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        approve(ctx, 1)?;

        // STEP 2: Freeze NFT token account
        let stake_data_seeds = &[
            b"stake_data".as_ref(),
            self.config.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[bumps.stake_data],
        ];
        let signer_seeds = &[&stake_data_seeds[..]];
        let metadata_program = &self.metadata_program.to_account_info();
        let delegate = &self.stake_data.to_account_info();
        let token_account = &self.user_ata_for_nft.to_account_info();
        let edition = &self.nft_edition.to_account_info();
        let mint = &self.nft_mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        // STEP 3: Save stake data
        self.stake_data.set_inner(StakeData {
            owner: self.user.key(),
            mint: self.nft_mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_data,
        });

        // STEP 4: Update user data
        self.user_data.staked_count += 1;

        Ok(())
    }
}
