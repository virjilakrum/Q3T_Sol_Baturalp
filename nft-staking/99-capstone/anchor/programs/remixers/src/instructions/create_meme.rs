use anchor_lang::prelude::*;

use crate::Meme;

#[derive(Accounts)]
#[instruction(seed: u32)]
pub struct CreateMeme<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + Meme::INIT_SPACE,
        seeds = [b"meme", seed.to_le_bytes().as_ref()],
        bump
    )]
    meme: Account<'info, Meme>,

    system_program: Program<'info, System>,
}

impl<'info> CreateMeme<'info> {
    pub fn create_meme(&mut self, seed: u32, bumps: &CreateMemeBumps) -> Result<()> {
        self.meme.set_inner(Meme {
            seed,
            maker: self.maker.key(),
            bump: bumps.meme,
        });
        msg!("✅ Created a meme.");
        Ok(())
    }
}
