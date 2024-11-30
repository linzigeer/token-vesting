use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::TREASURY_TOKEN_ACCOUNT;
use crate::state::VestingAccount;

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_bytes()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [TREASURY_TOKEN_ACCOUNT.as_bytes(), company_name.as_bytes()],
        bump,
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn create_vesting_account_iih(context: Context<CreateVestingAccount>, company_name: String) -> Result<()> {
    //全量初始化的时候建议用解引用
    //部分字段更新建议直接访问
    //逻辑复杂的更新建议用可变借用
    *context.accounts.vesting_account = VestingAccount {
        owner: context.accounts.signer.key(),
        mint: context.accounts.mint.key(),
        company_name,
        treasury_token_account: context.accounts.treasury_token_account.key(),
        treasury_bump: context.bumps.treasury_token_account,
        bump: context.bumps.vesting_account,
    };
    Ok(())
}

