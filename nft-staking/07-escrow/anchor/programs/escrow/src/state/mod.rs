use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowData {
    // Seed for this particular escrow
    pub seed: u64,

    // Alternatively we could include the maker's pubkey in the seeds and remove this line.
    // But having it inside the data is more convenient (helps with RPC queries).
    pub maker: Pubkey,

    // The token that the maker makes
    pub mint_a: Pubkey,

    // The token that the taker takes
    pub mint_b: Pubkey,

    // The amount of tokens that the taker takes (TODO)
    pub receive_amount: u64,

    // Bump for this account
    pub bump: u8,
}
