use crate::{errors, state::*, utils::now_ts};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use solana_safe_math::SafeMath;

#[derive(Accounts)]
#[instruction(bump_config_auth: u8, bump_reward_pot: u8)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Config account
    #[account(mut, has_one = config_authority)]
    pub config: Account<'info, ContractConfig>,
    /// CHECK: Config authority account
    #[account(
        seeds = [CONTRACT_CONFIG_SEED, config.key().as_ref()],
        bump = bump_config_auth
    )]
    pub config_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = user,
        seeds = [USER_STATE_SEED, user.to_account_info().key.as_ref()],
        bump
    )]
    pub user_state: Account<'info, User>,
    // Token storage
    #[account(
        mut,
        seeds = [CONTRACT_PDA_SEED, config.key().as_ref(), reward_mint.key().as_ref()],
        bump,
    )]
    pub reward_pot: Account<'info, TokenAccount>,
    pub reward_mint: Account<'info, Mint>,
    pub reward_destination: Account<'info, TokenAccount>,
    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Redeem<'info> {
    fn transfer_reward_token_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_pot.to_account_info(),
                to: self.reward_destination.to_account_info(),
                authority: self.config_authority.to_account_info(),
            },
        )
    }
}

fn calculate_reward<'info>(
    // All timestamps in seconds
    start_ts: u64,
    end_ts: u64,
    _user_state: &Account<'info, User>,
    config: &Account<'info, ContractConfig>,
) -> u64 {
    let mut reward: f32 = end_ts.safe_sub(start_ts).unwrap() as f32;
    reward *= config.multiplier;
    return reward as u64;
}

pub fn handler(ctx: Context<Redeem>, start_ts: u64, end_ts: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    let user_state = &ctx.accounts.user_state;

    if end_ts <= start_ts {
        return Err(error!(errors::ErrorCode::LockDurationInvalid));
    }

    if start_ts < config.start_ts {
        return Err(error!(errors::ErrorCode::LockContractNotStarted));
    }

    if end_ts > config.end_ts {
        return Err(error!(errors::ErrorCode::LockContractEnded));
    }

    let total_reward = calculate_reward(start_ts, end_ts, user_state, config);
    msg!("total reward: {}", total_reward);
    token::transfer(
        ctx.accounts
            .transfer_reward_token_ctx()
            .with_signer(&[&config.auth_seeds()]),
        total_reward,
    )?;

    let time_now = now_ts()?;
    let user_state = &mut ctx.accounts.user_state;
    user_state.time_last_redeemed = time_now;
    Ok(())
}
