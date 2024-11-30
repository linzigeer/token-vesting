use anchor_lang::prelude::*;
use crate::{EmployeeAccount, VestingAccount};
use crate::error_code::ErrorCode;
use crate::constants::EMPLOYEE_VESTING;

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    //数据的拥有者：数据由employer输入并签名，数据归属于employer
    #[account(mut)]
    pub owner: Signer<'info>,
    //代币归属计划收益人，应为员工的公钥
    pub beneficiary: SystemAccount<'info>,
    //公司归属计划账号，已经存在，只做约束（通过名字）
    #[account(has_one = owner)]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + EmployeeAccount::INIT_SPACE,
        seeds = [EMPLOYEE_VESTING.as_bytes(), beneficiary.key().as_ref(), vesting_account.key().as_ref()],
        bump,
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    pub system_program: Program<'info, System>,
}

pub fn create_employee_account_iih(
    ctx: Context<CreateEmployeeAccount>,
    start_time: i64,
    end_time: i64,
    cliff_time: i64,
    total_allocated_amount: u64,
) -> Result<()> {
    if end_time < start_time {
        return Err(ErrorCode::InvalidVestPeriod.into());
    }

    if cliff_time < start_time || cliff_time > end_time {
        return Err(ErrorCode::InvalidCliffTime.into());
    }


    //在首次全量初始化一个数据结构时建议使用解引用
    //在只修改少量字段时，建议直接访问
    //修改逻辑复杂时，建议使用可变借用
    *ctx.accounts.employee_account = EmployeeAccount {
        beneficiary: ctx.accounts.beneficiary.key(),
        start_time,
        end_time,
        cliff_time,
        total_allocated_amount,
        total_unlocked_amount: 0,
        total_withdrawn_amount: 0,
        vesting_account: ctx.accounts.vesting_account.key(),
        bump: ctx.bumps.employee_account,
    };
    Ok(())
}