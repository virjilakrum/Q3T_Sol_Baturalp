pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4vm6E7yANSLWdBFuPTvafnXY6q8v6XKRzRYSYghGrJV1");

#[program]
pub mod remixers {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }

    pub fn create_meme(ctx: Context<CreateMeme>, seed: u32) -> Result<()> {
        ctx.accounts.create_meme(seed, &ctx.bumps)
    }
}
