use anchor_lang::prelude::*;

declare_id!("4CCAExyY3ffuu3ZehyYHeeud8eiWKtaP6R7RCpfpDsia");

#[program]
pub mod solana_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
