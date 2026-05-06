use anchor_lang::prelude::*;

declare_id!("su3ABvPxbBB72HJd91GoezijKr7AW29wBYn5pFEfvrv");

mod constants;
mod errors;
mod events;
mod instructions;
mod maths;
mod state;

#[program]
pub mod cpmm_rust {
    use super::*;

    pub fn initialize_config(
        ctx: Context<instructions::initialize_config::IntializeConfig>,
    ) -> Result<()> {
        instructions::initialize_config::initialize_config(ctx)
    }
}
