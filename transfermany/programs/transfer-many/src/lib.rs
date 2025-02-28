use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    system_instruction::transfer,
};

declare_id!("CN3nYgvrJDnXYnrUNGZuVNjPTZjB78PvtxPoy2kgYBQG");

#[program]
pub mod transfer_many {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferMany>, recipients: Vec<Recipient>) -> Result<()> {
        let account= &ctx.accounts.from;
         
         let lamports:Vec<(Pubkey,u64)> = recipients.iter()
         .map(|r|(r.to,r.amount))
         .collect();
for (to, amount) in lamports.iter() {
            let ix = transfer(account.key, to, *amount);
            invoke(
                &ix,
                &[
                    account.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts,AnchorDeserialize,AnchorSerialize)]
pub struct TransferMany<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug,)]
pub struct Recipient{
    pub to:Pubkey,
    pub amount:u64,
}
#[error_code]
pub enum CustomError {
    #[msg("The number of recipients and amounts do not match.")]
    MismatchedLengths,
}
