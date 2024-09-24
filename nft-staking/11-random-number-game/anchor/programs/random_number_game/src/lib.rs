pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6HExArMT1U9NtFDNgaaoXEFdPQYg3ucicnQYAEQQCNig");

#[program]
pub mod random_number_game {
    use super::*;

    pub fn init(ctx: Context<Init>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        seed: u64,
        amount: u64,
        player_roll: u8,
    ) -> Result<()> {
        ctx.accounts
            .create_bet(seed, amount, player_roll, &ctx.bumps)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, signature: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&signature)?;
        ctx.accounts.resolve_bet(&signature, &ctx.bumps)
    }

    // TODO Implement "undo placing the bet"
}
