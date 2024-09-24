use anchor_lang::prelude::*;

use crate::state::UserData;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + UserData::INIT_SPACE,
        seeds = [b"user_data", user.key().as_ref()],
        bump
    )]
    user_data: Account<'info, UserData>,

    system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps) -> Result<()> {
        self.user_data.set_inner(UserData {
            points: 0,
            staked_count: 0,
            bump: bumps.user_data,
        });
        Ok(())
    }
}
