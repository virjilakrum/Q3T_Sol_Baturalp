pub mod contexts {
    use super::*;

    #[derive(Accounts)]
    pub struct Init<'info> {
        #[account(init, payer = authority, space = 8 + Config::LEN)]
        pub config: Account<'info, Config>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct InitUser<'info> {
        #[account(init, payer = user, space = 8 + UserState::LEN)]
        pub user_state: Account<'info, UserState>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Stake<'info> {
        #[account(mut)]
        pub user_state: Account<'info, UserState>,
        #[account(mut)]
        pub nft_mint: Account<'info, Mint>,
        #[account(mut)]
        pub nft_token_account: Account<'info, TokenAccount>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
        #[account(mut)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct Unstake<'info> {
        #[account(mut)]
        pub user_state: Account<'info, UserState>,
        #[account(mut)]
        pub nft_mint: Account<'info, Mint>,
        #[account(mut)]
        pub nft_token_account: Account<'info, TokenAccount>,
        pub token_program: Program<'info, Token>,
        #[account(mut)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct Claim<'info> {
        #[account(mut)]
        pub user_state: Account<'info, UserState>,
        #[account(mut)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct UpdateConfig<'info> {
        #[account(mut)]
        pub config: Account<'info, Config>,
        #[account(mut)]
        pub authority: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct CloseUserAccount<'info> {
        #[account(mut, close = user)]
        pub user_state: Account<'info, UserState>,
        #[account(mut)]
        pub user: Signer<'info>,
    }
}


