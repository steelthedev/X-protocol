use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub fee_recipient: Pubkey,
    pub market_count: u64,
    pub default_fee_bps: u16,
    pub bump: u8,
    pub reserved: [u8; 64],
}
