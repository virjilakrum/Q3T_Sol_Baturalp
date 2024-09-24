pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("67pguSETAjZra9HoX9g7CvLdFYtn9U8CHZhxUrRFs4dj");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn init(ctx: Context<Init>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    // TODO Implement purchase
    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {}
}
