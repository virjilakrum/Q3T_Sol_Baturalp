use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Bet;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    player: Signer<'info>,

    // TODO Instead of this we could get the house pubkey as an instruction argument. Try that!
    /// CHECK: This is fine.
    house: UncheckedAccount<'info>,

    #[account(
        // mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,

    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), player.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    bet: Account<'info, Bet>,

    system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        seed: u64,
        amount: u64,
        player_roll: u8,
        bumps: &PlaceBetBumps,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            player: self.player.key(),
            seed,
            amount,
            player_roll,
            slot: Clock::get()?.slot,
            bump: bumps.bet,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)
    }
}
