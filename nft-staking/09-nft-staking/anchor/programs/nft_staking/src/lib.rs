use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;
pub mod errors;
pub mod state;

declare_id!("FTyBtbmmdwLvGXUsJSJqRQkkPpL63L5yLjSakJGrvvYZ");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        points_per_stake: u8,
        max_stakes: u8,
        freeze_min_days: u16,
    ) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stakes, freeze_min_days, &ctx.bumps)
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }


    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_points_per_stake: Option<u8>,
        new_max_stakes: Option<u8>,
        new_freeze_min_days: Option<u16>,
    ) -> Result<()> {
        ctx.accounts.update_config(new_points_per_stake, new_max_stakes, new_freeze_min_days)
    }

    pub fn close_user_account(ctx: Context<CloseUserAccount>) -> Result<()> {
        ctx.accounts.close_user_account()
    }
}

