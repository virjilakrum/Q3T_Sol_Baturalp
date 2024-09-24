pub mod state {
    use super::*;

    #[account]
    pub struct Config {
        pub authority: Pubkey,
        pub points_per_stake: u8,
        pub max_stakes: u8,
        pub freeze_min_days: u16,
    }

    #[account]
    pub struct UserState {
        pub user: Pubkey,
        pub staked_nfts: Vec<Pubkey>,
        pub points: u64,
        pub last_stake_timestamp: i64,
    }

    impl Config {
        pub const LEN: usize = 32 + 1 + 1 + 2;
    }

    impl UserState {
        pub const LEN: usize = 32 + 32 * 5 + 8 + 8; // Assume max 5 NFTs can be staked
    }
}