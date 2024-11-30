use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    #[max_len(128)]
    pub company_name: String,
    pub treasury_token_account: Pubkey,
    pub treasury_bump: u8,
    pub bump: u8,
}