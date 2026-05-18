use anchor_lang::prelude::*;

pub mod state;

declare_id!("B2fbWZXvAZGdjBV7bwt5LzsD5VPwwf6errc134sVHzdr");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
