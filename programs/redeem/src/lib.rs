use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("DZDZu44QG5jnKzF87hjpLSt3djVsA2uGKMED9FYn92wn");

#[program]
pub mod tohken {
    use super::*;

    pub fn init_contract_config(
        ctx: Context<InitContractConfig>,
        bump_config_auth: u8,
        multiplier: f32,
        start_ts: u64,
        end_ts: u64,
        max_redeems: u64,
    ) -> Result<()> {
        instructions::init_contract_config::handler(
            ctx,
            bump_config_auth,
            multiplier,
            start_ts,
            end_ts,
            max_redeems,
        )
    }

    pub fn redeem(
        ctx: Context<Redeem>,
        _bump_config_auth: u8,
        _bump_reward_pot: u8,
        start_ts: u64,
        end_ts: u64,
    ) -> Result<()> {
        instructions::redeem::handler(ctx, start_ts, end_ts)
    }
}
