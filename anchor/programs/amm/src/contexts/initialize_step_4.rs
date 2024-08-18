use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::state::AmmConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeStep4<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"amm_config", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, AmmConfig>,

    mint_x: InterfaceAccount<'info, Mint>,

    mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program,
        seeds = [b"amm_mint_lp", config.key().as_ref()],
        bump = config.mint_lp_bump
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_x,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_y,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_lp: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,

    associated_token_program: Program<'info, AssociatedToken>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeStep4<'info> {
    pub fn deposit(&mut self, amount: u64, is_x: bool) -> Result<()> {
        let (from, mint, to, decimals) = match is_x {
            true => (
                self.maker_ata_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.maker_ata_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };
        let accounts = TransferChecked {
            from,
            mint,
            to,
            authority: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, decimals)
    }

    pub fn mint_lp_tokens(&mut self, amount_x: u64, amount_y: u64) -> Result<()> {
        let amount_lp = amount_x
            .checked_mul(amount_y)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let seed = self.config.seed.to_le_bytes();
        let bump = [self.config.bump];
        let signer_seeds = [&[
            b"amm_config",
            self.mint_x.to_account_info().key.as_ref(),
            self.mint_y.to_account_info().key.as_ref(),
            seed.as_ref(),
            &bump,
        ][..]];

        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        mint_to(ctx, amount_lp)
    }

    pub fn complete_init_step_4(&mut self) -> Result<()> {
        if self.config.init_state != 3 {
            panic!(); // TODO
        }
        self.config.init_state = 4;
        Ok(())
    }
}
