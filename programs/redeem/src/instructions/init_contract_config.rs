use crate::state::contract_config::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(bump_config_auth: u8)]
pub struct InitContractConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub reward_mint: Account<'info, Mint>,
    // Config account
    #[account(
        init,
        payer = admin,
        space = 8 + std::mem::size_of::<ContractConfig>(),
    )]
    pub config: Account<'info, ContractConfig>,
    #[account(
        seeds = [CONTRACT_CONFIG_SEED, config.key().as_ref()],
        bump
    )]
    /// CHECK: Config authority account
    pub config_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub destination: UncheckedAccount<'info>,
    // Token storage
    #[account(
        init,
        seeds = [CONTRACT_PDA_SEED, config.key().as_ref(), reward_mint.key().as_ref()],
        bump,
        payer = admin,
        token::mint = reward_mint,
        token::authority = config_authority,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitContractConfig>,
    bump_config_auth: u8,
    multiplier: f32,
    start_ts: u64,
    end_ts: u64,
    max_redeems: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.admin = ctx.accounts.admin.key();
    config.max_redeems = max_redeems;
    config.start_ts = start_ts;
    config.end_ts = end_ts;
    config.is_active = true;
    config.multiplier = multiplier;
    config.mint = ctx.accounts.reward_mint.to_account_info().key();
    config.config_authority = ctx.accounts.config_authority.key();
    config.config_authority_seed = config.key();
    config.config_authority_bump_seed = [bump_config_auth];

    Ok(())
}
