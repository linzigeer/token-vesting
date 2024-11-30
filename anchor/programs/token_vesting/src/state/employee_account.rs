use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    //归属受益人
    pub beneficiary: Pubkey,
    //归属开始时间
    pub start_time: i64,
    //归属结束时间
    pub end_time: i64,
    //归属断崖时间
    pub cliff_time: i64,
    //代币分配数量
    pub total_allocated_amount: u64,
    //代币解锁数量
    pub total_unlocked_amount: u64,
    //代币已提取数量
    pub total_withdrawn_amount: u64,
    //所属公司归属账号
    pub vesting_account: Pubkey,
    pub bump: u8,

    
}