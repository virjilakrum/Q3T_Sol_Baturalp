use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;

declare_id!("DiGxbunBs6DojD9ns1J9vc8Z6YkYPYcocG2xAJpm4dnz");

#[program]
pub mod amm {
    use super::*;

    // Ensures initialization of config and LP mint accounts
    pub fn initialize_step_1(ctx: Context<InitializeStep1>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts
            .complete_init_step_1(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)
    }

    // Ensures initialization of vault_x and vault_y accounts
    pub fn initialize_step_2(ctx: Context<InitializeStep2>) -> Result<()> {
        ctx.accounts.complete_init_step_2()
    }

    // Ensures initialization of maker_ata_x, maker_ata_y and maker_ata_lp accounts
    pub fn initialize_step_3(ctx: Context<InitializeStep3>) -> Result<()> {
        ctx.accounts.complete_init_step_3()
    }

    // Ensures initial liquidity and finishes initialization
    pub fn initialize_step_4(
        ctx: Context<InitializeStep4>,
        amount_x: u64,
        amount_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount_x, true)?;
        ctx.accounts.deposit(amount_y, false)?;
        ctx.accounts.mint_lp_tokens(amount_x, amount_y)?;
        ctx.accounts.complete_init_step_4()
    }

    // Add liquidity to mint LP tokens
    // pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
    //     // deposit_tokens(amount)?;
    //     // mint_lp_token(amount)
    //     Ok(())
    // }

    // // Burn LP tokens to withdraw liquidity
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
    //     // burn_lp_token(amount)?;
    //     // withdraw_tokens(amount)
    //     Ok(())
    // }

    // pub fn swap(
    //     ctx: Context<Swap>,
    //     amount: u64,
    //     min_receive: u64,
    //     is_x: bool, /*, expiration: i64 */
    // ) -> Result<()> {
    //     // deposit_token()?;
    //     // withdraw_token()
    //     Ok(())
    // }
}
