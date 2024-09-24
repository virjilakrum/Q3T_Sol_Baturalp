use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
};

use crate::errors::CustomErrorCode;
use crate::state::{Config, StakeData, UserData};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    user: Signer<'info>,

    // TODO Is this annotation not needed?
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
        mut,
        close = user,
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
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    nft_edition: Account<'info, MasterEditionAccount>,

    #[account(
        // TODO Is mut necessary here?
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    user_ata_for_nft: Account<'info, TokenAccount>,

    metadata_program: Program<'info, Metadata>,

    token_program: Program<'info, Token>,
    // system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // STEP 0: Check freeze period
        let days_elapsed =
            (Clock::get()?.unix_timestamp - self.stake_data.staked_at) / (60 * 60 * 24);
        require!(
            days_elapsed >= self.config.freeze_min_days.into(),
            CustomErrorCode::FreezePeriodNotOver
        );

        // STEP 1: Thaw NFT token account
        let stake_data_seeds = &[
            b"stake_data".as_ref(),
            self.config.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.stake_data.bump],
        ];
        let signer_seeds = &[&stake_data_seeds[..]];
        let metadata_program = &self.metadata_program.to_account_info();
        let delegate = &self.stake_data.to_account_info();
        let token_account = &self.user_ata_for_nft.to_account_info();
        let edition = &self.nft_edition.to_account_info();
        let mint = &self.nft_mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        // STEP 2: Revoke (nullify) delegation
        let accounts = Revoke {
            source: self.user_ata_for_nft.to_account_info(), // The account that holds the tokens
            authority: self.user.to_account_info(), // The account that can use the tokens (the owner)
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        revoke(ctx)?;

        // STEP 3: Update user data
        self.user_data.staked_count -= 1;
        self.user_data.points += days_elapsed as u32 * self.config.points_per_day as u32;

        Ok(())
    }
}
