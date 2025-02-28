use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed,
    system_instruction::create_account_with_seed,
};
use serde::{Deserialize, Serialize};

declare_id!("TCJikbbbyU65XnjPniSsoPuo2fGgj6DjweJ8RRQ6KCq");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, space: u64, amount: u64, seed: String) -> Result<()> {
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let base = &ctx.accounts.base;
        let sys_pro = &ctx.accounts.system_program;

        msg!("Creating account with seed...");
        msg!("Base: {}", base.key());
        msg!("Seed: {}", seed);

        let (expected_pda, bump) = Pubkey::find_program_address(&[seed.as_bytes(), base.key().as_ref()], &ctx.program_id);
        msg!("Expected PDA: {}", expected_pda);
        msg!("Provided PDA: {}", to.key());

        require_keys_eq!(expected_pda, to.key(), CustomError::PDA_Mismatch);

        let rent_exempt_amount = Rent::get()?.minimum_balance(space as usize);

        let ix = create_account_with_seed(
            &from.key(),
            &to.key(),
            &base.key(),
            &seed,
            rent_exempt_amount.max(amount),
            space,
            &ctx.program_id,
        );

        let seeds = &[seed.as_bytes(), base.key().as_ref(), &[bump]];

        invoke_signed(
            &ix,
            &[
                from.to_account_info(),
                to.to_account_info(),
                base.to_account_info(),
                sys_pro.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(space: u64, amount: u64, seed: String)] 
pub struct Initialize<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>, // ✅ Changed from SystemAccount to AccountInfo

    pub base: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("PDA does not match the expected PDA")]
    PDA_Mismatch,
}
