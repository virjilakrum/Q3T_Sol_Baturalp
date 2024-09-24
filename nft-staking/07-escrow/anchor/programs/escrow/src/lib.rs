use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;

declare_id!("F9tzHfxVjtU7nRTZ4v1JphpRK932kFdz5gbtZvmNsXyB");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts
            .save_escrow_data(seed, receive_amount, ctx.bumps.escrow_data)?;
        ctx.accounts.deposit_to_escrow(amount) // ^maker -> escrow
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.pay_the_maker()?; // ^taker -> maker
        ctx.accounts.withdraw_and_close() // escrow+rent -> ^taker
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close() // escrow+rent -> ^maker
    }
}
