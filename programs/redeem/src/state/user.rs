use anchor_lang::prelude::*;

pub const USER_STATE_SEED: &[u8] = b"user-state";

#[account]
pub struct User {
    pub user: Pubkey,
    pub time_last_redeemed: u64,
}
