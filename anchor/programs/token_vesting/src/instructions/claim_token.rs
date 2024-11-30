use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use crate::{EmployeeAccount, VestingAccount};
use crate::error_code::ErrorCode;
use crate::constants::{TREASURY_TOKEN_ACCOUNT, EMPLOYEE_VESTING};

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimToken<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    //员工索取代币时，国库代币金额会减少，需要施加mut约束
    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [company_name.as_bytes()],
        bump = vesting_account.bump,
        has_one = mint,
        has_one = treasury_token_account,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    //需要修改员工账号中的total_unlock_amount、total_withdrawn_amount
    #[account(
        mut,
        seeds = [EMPLOYEE_VESTING.as_bytes(), beneficiary.key().as_ref(), vesting_account.key().as_ref()],
        bump = employee_account.bump,
        has_one = beneficiary,
        has_one = vesting_account,
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program,
    )]
    pub employee_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn claim_token_iih(ctx: Context<ClaimToken>, _company_name: String) -> Result<()> {
    let employee_account = &mut ctx.accounts.employee_account;
    let now = Clock::get()?.unix_timestamp;

    let cliff_time = employee_account.cliff_time;
    //还没到claim时间
    if now <= cliff_time {
        return Err(ErrorCode::ClaimTimeNotReachedYet.into());
    }

    let company_name_as_bytes = ctx.accounts.vesting_account.company_name.as_bytes();
    let start_time = employee_account.start_time;
    let end_time = employee_account.end_time;
    let time_since_start = now.saturating_sub(start_time);
    let total_vesting_time = end_time.saturating_sub(start_time);
    let total_allocated_amount = employee_account.total_allocated_amount;
    let total_withdrawn_amount = employee_account.total_withdrawn_amount;

    let current_total_unlock_amount = if now >= end_time {
        total_allocated_amount
    } else {
        (total_allocated_amount * time_since_start as u64) / total_vesting_time as u64
    };

    let current_claimable_amount = current_total_unlock_amount.saturating_sub(total_withdrawn_amount);

    if current_claimable_amount == 0 {
        return Err(ErrorCode::ClaimNotAvailableNow.into());
    }

    let cpi_transfer_checked = TransferChecked {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.employee_token_account.to_account_info(),
        authority: ctx.accounts.treasury_token_account.to_account_info(),
    };

    let token_program = ctx.accounts.token_program.to_account_info();
    let signer_seeds: &[&[&[u8]]] = &[
        &[
            TREASURY_TOKEN_ACCOUNT.as_bytes(),
            company_name_as_bytes,
            &[ctx.accounts.vesting_account.treasury_bump]
        ]
    ];

    let cpi_context = CpiContext::new_with_signer(token_program, cpi_transfer_checked, signer_seeds);

    let decimals = ctx.accounts.mint.decimals;

    transfer_checked(cpi_context, current_claimable_amount, decimals)?;

    employee_account.total_unlocked_amount = current_total_unlock_amount;
    employee_account.total_withdrawn_amount += current_claimable_amount;

    Ok(())
}