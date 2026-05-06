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

pub fn initialize_config(ctx: Context<IntializeConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.admin = ctx.accounts.admin.key();
    config.fee_recipient = ctx.accounts.fee_recipient.key();
    config.market_count = 0;
    config.bump = ctx.bumps.config;
    config.default_fee_bps = 200;

    Ok(())
}
