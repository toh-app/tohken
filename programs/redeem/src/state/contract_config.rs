use anchor_lang::prelude::*;

pub const CONTRACT_CONFIG_SEED: &[u8] = b"contract-config";
pub const CONTRACT_PDA_SEED: &[u8] = b"contract-pda";

#[account]
pub struct ContractConfig {
    /// Admin (with tokens)
    pub admin: Pubkey,
    // Max number of redemptions
    pub max_redeems: u64,
    // Start and end timestamps for contract
    pub start_ts: u64,
    pub end_ts: u64,
    pub is_active: bool,
    // Multiplier for points. Base is 1h = 1 token
    pub multiplier: f32,
    // Token
    pub mint: Pubkey,
    pub config_authority: Pubkey,
    pub config_authority_seed: Pubkey,
    pub config_authority_bump_seed: [u8; 1],
}

impl ContractConfig {
    pub fn auth_seeds(&self) -> [&[u8]; 3] {
        [
            CONTRACT_CONFIG_SEED,
            self.config_authority_seed.as_ref(),
            &self.config_authority_bump_seed,
        ]
    }
}
