use anchor_lang::prelude::*;

declare_id!("BhE35ptgBkeJMPo9zo6YLzPmWKuck88BHGekC3h6XiyE");

#[allow(unexpected_cfgs)]

pub mod instructions;
pub mod constants;
pub mod errors;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;




#[program]
mod token_vesting {
    use super::*;

    pub fn create_vesting_account_oih(
        ctx: Context<CreateVestingAccount>,
        company_name: String
    ) -> Result<()> {
        instructions::create_vesting_account_iih(ctx, company_name)?;
        Ok(())
    }

    pub fn create_employee_account_oih(
        ctx: Context<CreateEmployeeAccount>,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        total_allocated_amount: u64,
    ) -> Result<()> {
        instructions::create_employee_account_iih(ctx, start_time, end_time, cliff_time, total_allocated_amount)?;
        Ok(())
    }

    pub fn claim_token_oih(
        ctx: Context<ClaimToken>,
        company_name: String,
    ) -> Result<()> {
        instructions::claim_token_iih(ctx, company_name)?;
        Ok(())
    }
}


