use anchor_lang::prelude::*;

use crate::state::config::Config;

#[derive(Accounts)]
pub struct IntializeConfig<'info> {
    #[account(
        init,
        payer = deployer,
        seeds = [b"config"],
        bump,
        space = Config::INIT_SPACE + 8,

    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub deployer: Signer<'info>,

    pub admin: UncheckedAccount<'info>,

    pub fee_recipient: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
