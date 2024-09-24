use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::{error::CustomErrorCode, Bet};

const HOUSE_EDGE: u16 = 150; // 1.5%

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    house: Signer<'info>,

    /// CHECK: This is fine.
    player: UncheckedAccount<'info>,

    #[account(
        // mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), player.key().as_ref(), bet.seed.to_le_bytes().as_ref()], // TODO Does this make sense?
        bump = bet.bump
    )]
    bet: Account<'info, Bet>,

    /// CHECK: This is fine.
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    instruction_sysvar: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, _sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;

        require_keys_eq!(
            ix.program_id,
            ed25519_program::ID,
            CustomErrorCode::Ed25519ProgramError
        );

        require_eq!(ix.accounts.len(), 0, CustomErrorCode::Ed25519AccountsError);

        let sigs_vec = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(sigs_vec.len(), 1, CustomErrorCode::Ed25519DataLengthError);

        let sig = &sigs_vec[0];

        require!(sig.is_verifiable, CustomErrorCode::Ed25519HeaderError);

        require_keys_eq!(
            sig.public_key.ok_or(CustomErrorCode::Ed25519PubkeyError)?,
            self.house.key(),
            CustomErrorCode::Ed25519PubkeyError
        );

        // require_eq!(
        //     &sig.signature
        //         .ok_or(CustomErrorCode::Ed25519SignatureError)?,
        //     _sig,
        //     CustomErrorCode::Ed25519SignatureError
        // );
        require!(
            &sig.signature
                .ok_or(CustomErrorCode::Ed25519SignatureError)?
                .eq(_sig),
            CustomErrorCode::Ed25519SignatureError
        );

        // require_eq!(
        //     &sig.message.ok_or(CustomErrorCode::Ed25519SignatureError)?,
        //     &self.bet.to_slice(),
        //     CustomErrorCode::Ed25519SignatureError
        // );
        require!(
            &sig.message
                .as_ref()
                .ok_or(CustomErrorCode::Ed25519SignatureError)?
                .eq(&self.bet.to_slice()),
            CustomErrorCode::Ed25519SignatureError
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16 = [0u8; 16];

        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let house_roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.player_roll > house_roll {
            let payout_amount = (self.bet.amount as u128)
                .checked_mul(10000 - HOUSE_EDGE as u128)
                .ok_or(CustomErrorCode::OverflowError)?
                .checked_div(self.bet.player_roll as u128 - 1)
                .ok_or(CustomErrorCode::OverflowError)?
                .checked_div(100)
                .ok_or(CustomErrorCode::OverflowError)? as u64;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
            let vault_seeds = [
                b"vault".as_ref(),
                &self.house.key().to_bytes()[..],
                &[bumps.vault],
            ];
            let signer_seeds = &[&vault_seeds[..]][..];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds,
            );
            transfer(ctx, payout_amount)?;
        }
        // TODO else

        Ok(())
    }
}
