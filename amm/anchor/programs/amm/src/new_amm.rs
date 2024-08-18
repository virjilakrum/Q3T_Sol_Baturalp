use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::collections::HashMap;
declare_id!("FTyBtbmmdwLvGXUsJSJqRQkkPpL63L5yLjSakJGrvvYZ");

pub mod contexts;
pub use contexts::*;
pub mod state;
use state::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        emergency_owner: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.seed = seed;
        config.fee = fee;
        config.emergency_owner = emergency_owner;
        config.paused = false;
        config.whitelist = HashMap::new();
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_x: u64,
        amount_y: u64,
        min_lp: u64,
        expiration: i64,
    ) -> Result<()> {
        require!(!ctx.accounts.config.paused, ErrorCode::ContractPaused);
        require!(
            Clock::get()?.unix_timestamp <= expiration,
            ErrorCode::TransactionExpired
        );

        let (lp_to_mint, x_to_deposit, y_to_deposit) = calculate_liquidity_amounts(
            amount_x,
            amount_y,
            ctx.accounts.pool_x.amount,
            ctx.accounts.pool_y.amount,
            ctx.accounts.lp_mint.supply,
        )?;

        require!(lp_to_mint >= min_lp, ErrorCode::SlippageExceeded);

        // Transfer tokens to the pool
        token::transfer(ctx.accounts.into_transfer_x_context(), x_to_deposit)?;
        token::transfer(ctx.accounts.into_transfer_y_context(), y_to_deposit)?;

        // Mint LP tokens
        token::mint_to(ctx.accounts.into_mint_lp_context(), lp_to_mint)?;

        emit!(LiquidityAdded {
            user: ctx.accounts.user.key(),
            amount_x: x_to_deposit,
            amount_y: y_to_deposit,
            lp_minted: lp_to_mint,
        });

        Ok(())
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_amount: u64,
        min_x: u64,
        min_y: u64,
        expiration: i64,
    ) -> Result<()> {
        require!(!ctx.accounts.config.paused, ErrorCode::ContractPaused);
        require!(
            Clock::get()?.unix_timestamp <= expiration,
            ErrorCode::TransactionExpired
        );

        let (x_to_withdraw, y_to_withdraw) = calculate_withdrawal_amounts(
            lp_amount,
            ctx.accounts.pool_x.amount,
            ctx.accounts.pool_y.amount,
            ctx.accounts.lp_mint.supply,
        )?;

        require!(
            x_to_withdraw >= min_x && y_to_withdraw >= min_y,
            ErrorCode::SlippageExceeded
        );

        // Burn LP tokens
        token::burn(ctx.accounts.into_burn_lp_context(), lp_amount)?;

        // Transfer tokens from the pool
        token::transfer(
            ctx.accounts.into_transfer_x_from_pool_context(),
            x_to_withdraw,
        )?;
        token::transfer(
            ctx.accounts.into_transfer_y_from_pool_context(),
            y_to_withdraw,
        )?;

        emit!(LiquidityRemoved {
            user: ctx.accounts.user.key(),
            amount_x: x_to_withdraw,
            amount_y: y_to_withdraw,
            lp_burned: lp_amount,
        });

        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
        min_amount_out: u64,
        is_x_to_y: bool,
        expiration: i64,
    ) -> Result<()> {
        require!(!ctx.accounts.config.paused, ErrorCode::ContractPaused);
        require!(
            Clock::get()?.unix_timestamp <= expiration,
            ErrorCode::TransactionExpired
        );

        let (amount_out, fee) = calculate_swap_amount(
            amount_in,
            ctx.accounts.pool_x.amount,
            ctx.accounts.pool_y.amount,
            ctx.accounts.config.fee,
            is_x_to_y,
        )?;

        require!(amount_out >= min_amount_out, ErrorCode::SlippageExceeded);

        // Transfer tokens in
        if is_x_to_y {
            token::transfer(ctx.accounts.into_transfer_x_context(), amount_in)?;
            token::transfer(ctx.accounts.into_transfer_y_from_pool_context(), amount_out)?;
        } else {
            token::transfer(ctx.accounts.into_transfer_y_context(), amount_in)?;
            token::transfer(ctx.accounts.into_transfer_x_from_pool_context(), amount_out)?;
        }

        emit!(Swapped {
            user: ctx.accounts.user.key(),
            amount_in,
            amount_out,
            is_x_to_y,
            fee,
        });

        Ok(())
    }

    pub fn update_fee(ctx: Context<UpdateFee>, new_fee: u16) -> Result<()> {
        require!(new_fee <= 10000, ErrorCode::InvalidFee); // Max fee is 100%
        ctx.accounts.config.fee = new_fee;
        emit!(FeeUpdated { new_fee });
        Ok(())
    }

    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        ctx.accounts.config.paused = !ctx.accounts.config.paused;
        emit!(PauseToggled {
            paused: ctx.accounts.config.paused
        });
        Ok(())
    }

    pub fn update_whitelist(
        ctx: Context<UpdateWhitelist>,
        token: Pubkey,
        is_whitelisted: bool,
    ) -> Result<()> {
        ctx.accounts.config.whitelist.insert(token, is_whitelisted);
        emit!(WhitelistUpdated {
            token,
            is_whitelisted
        });
        Ok(())
    }

    pub fn get_price(ctx: Context<GetPrice>) -> Result<u64> {
        let price = (ctx.accounts.pool_y.amount as u128)
            .checked_mul(10u128.pow(6))
            .unwrap()
            .checked_div(ctx.accounts.pool_x.amount as u128)
            .unwrap() as u64;

        emit!(PriceQueried { price });
        Ok(price)
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + Config::LEN, seeds = [b"config", seed.to_le_bytes().as_ref()], bump)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub pool_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_y: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_y: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub pool_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_y: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_y: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub pool_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_y: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_x: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_y: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut, has_one = emergency_owner)]
    pub config: Account<'info, Config>,
    pub emergency_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(mut, has_one = emergency_owner)]
    pub config: Account<'info, Config>,
    pub emergency_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateWhitelist<'info> {
    #[account(mut, has_one = emergency_owner)]
    pub config: Account<'info, Config>,
    pub emergency_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPrice<'info> {
    pub config: Account<'info, Config>,
    pub pool_x: Account<'info, TokenAccount>,
    pub pool_y: Account<'info, TokenAccount>,
}

#[account]
pub struct Config {
    pub seed: u64,
    pub fee: u16,
    pub emergency_owner: Pubkey,
    pub paused: bool,
    pub whitelist: HashMap<Pubkey, bool>,
}

impl Config {
    pub const LEN: usize = 8 + 32 + 2 + 32 + 1 + 32; // Approximate size
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided fee is invalid")]
    InvalidFee,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("The contract is paused")]
    ContractPaused,
    #[msg("The transaction has expired")]
    TransactionExpired,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
}

#[event]
pub struct LiquidityAdded {
    pub user: Pubkey,
    pub amount_x: u64,
    pub amount_y: u64,
    pub lp_minted: u64,
}

#[event]
pub struct LiquidityRemoved {
    pub user: Pubkey,
    pub amount_x: u64,
    pub amount_y: u64,
    pub lp_burned: u64,
}

#[event]
pub struct Swapped {
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub is_x_to_y: bool,
    pub fee: u64,
}

#[event]
pub struct FeeUpdated {
    pub new_fee: u16,
}

#[event]
pub struct PauseToggled {
    pub paused: bool,
}

#[event]
pub struct WhitelistUpdated {
    pub token: Pubkey,
    pub is_whitelisted: bool,
}

#[event]
pub struct PriceQueried {
    pub price: u64,
}
