use anchor_lang::prelude::*;

declare_id!("CqNsAszt2UjSShujcPiU6kY6uKX149uodSDBapcaiKiN");

#[program]
pub mod vault {
    use super::*;
    use anchor_lang::system_program::{transfer, Transfer};

    pub fn initialize(ctx: Context<InitializeAccs>) -> Result<()> {
        let vault_state = &mut ctx.accounts.vault_state;
        vault_state.vault_state_bump = ctx.bumps.vault_state;
        vault_state.vault_bump = ctx.bumps.vault;
        vault_state.owner = ctx.accounts.user.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositOrWithdrawAccs>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        emit!(DepositEvent {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<DepositOrWithdrawAccs>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        require!(
            ctx.accounts.vault.lamports() >= amount,
            VaultError::InsufficientFunds
        );

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let vault_seeds = &[
            b"vault",
            ctx.accounts.vault_state.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        emit!(WithdrawEvent {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }

    pub fn close(ctx: Context<CloseAccs>) -> Result<()> {
        let amount = ctx.accounts.vault.lamports();
        require!(amount > 0, VaultError::EmptyVault);

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let vault_seeds = &[
            b"vault",
            ctx.accounts.vault_state.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        emit!(CloseEvent {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"vault_state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositOrWithdrawAccs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.vault_state_bump,
        constraint = vault_state.owner == user.key() @ VaultError::Unauthorized
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseAccs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.vault_state_bump,
        constraint = vault_state.owner == user.key() @ VaultError::Unauthorized
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub vault_state_bump: u8,
    pub vault_bump: u8,
    pub owner: Pubkey,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1 + 32;
}

#[error_code]
pub enum VaultError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Vault is empty")]
    EmptyVault,
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct CloseEvent {
    pub user: Pubkey,
    pub amount: u64,
}
