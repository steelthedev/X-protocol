use anchor_lang::prelude::*;

declare_id!("FTsZ6dsJPgtAHKHy6ECLCqMA541bDkC9gXKnAzvoaF6j");

mod constants;
mod errors;
mod events;
mod instructions;
mod maths;
mod state;

#[program]
pub mod cpmm_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
