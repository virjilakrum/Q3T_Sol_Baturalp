use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u64,
    pub amount: u64,
    pub player_roll: u8,
    pub slot: u64,
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        // let mut result = self.player.to_bytes().to_vec();
        let mut result = vec![];
        result.extend_from_slice(&self.player.to_bytes());
        result.extend_from_slice(&self.seed.to_le_bytes());
        result.extend_from_slice(&self.amount.to_le_bytes());
        result.extend_from_slice(&self.player_roll.to_le_bytes());
        result.extend_from_slice(&self.slot.to_le_bytes());
        result.extend_from_slice(&self.bump.to_le_bytes());
        result
    }
}
