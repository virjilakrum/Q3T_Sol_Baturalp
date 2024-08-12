use anchor_lang::prelude::*;

declare_id!("9cbay1PRQ36j647T1YeuXfjCpCBipT4hh3Y21PUP3jQt");

#[program]
pub mod vault {
    use super::*;
    use anchor_lang::system_program::{transfer, Transfer};

    pub fn initialize(ctx: Context<InitializeAccs>) -> Result<()> {
        ctx.accounts.vault_state.vault_state_bump = ctx.bumps.vault_state;
        ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;

        Ok(())
    }

    pub fn deposit(ctx: Context<DepositOrWithdrawAccs>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(ctx: Context<DepositOrWithdrawAccs>, amount: u64) -> Result<()> {
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

        transfer(cpi_ctx, amount)
    }

    pub fn close(ctx: Context<CloseAccs>) -> Result<()> {
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

        let amount = ctx.accounts.vault.lamports();
        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct InitializeAccs<'info> {
    // The signer of the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    // The PDA that will store the state (currently just bumps) for this particular user
    #[account(
        init,
        payer = user,
        seeds = [b"vault_state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    // The PDA that will store the lamports for this particular user
    // Even though it doesn't exist yet, we don't init it. Because it can be initted when the user deposits into the vault for the first time
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // Needed for creating accounts and transferring lamports
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositOrWithdrawAccs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.vault_state_bump
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
        bump = vault_state.vault_state_bump
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
    // Bump used for creating this (vault state) account
    pub vault_state_bump: u8,
    // Bump used for creating the account where the lamports are stored
    pub vault_bump: u8,
}
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}
