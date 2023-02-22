use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke, program_error::ProgramError, system_instruction,
};
declare_id!("zzEPkxAU9v1QEFGj23FBErhWSLdXJcZpBXTaMG8Vp1o");

#[program]
pub mod sec3_watchtower_demo {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let reserve = &mut ctx.accounts.reserve;
        reserve.authority = ctx.accounts.authority.key();
        msg!("reserve.authority: {}", reserve.authority);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let reserve_balance = **ctx.accounts.reserve.to_account_info().lamports.borrow_mut();
        msg!("reserve_balance: {}", reserve_balance);
        msg!("deposit_amount: {}", amount);
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.source.key(),
                &ctx.accounts.reserve.key(),
                amount,
            ),
            &[
                ctx.accounts.reserve.to_account_info().clone(),
                ctx.accounts.source.to_account_info().clone(),
            ],
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let reserve_balance = **ctx.accounts.reserve.to_account_info().lamports.borrow_mut();
        msg!("reserve_balance: {}", reserve_balance);
        msg!("withdraw_amount: {}", amount);

        if amount > 10 {
            return Err(ErrorCode::OverWithdrawLimit.into());
        }
        if amount > reserve_balance {
            return Err(ProgramError::InsufficientFunds.into());
        }

        **ctx.accounts.reserve.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.destination.lamports.borrow_mut() += amount;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        let reserve_key = ctx.accounts.reserve.to_account_info().key;
        msg!("closed reserve: {}", reserve_key);
        Ok(())
    }
}

#[account]
#[repr(C, align(8))]
#[derive(Default)]
pub struct Reserve {
    pub authority: Pubkey,
}

impl Reserve {
    pub const SIZE: usize = 1 + 32 + 32 + 8;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [ authority.key.as_ref() ],
        bump,
        payer = authority,
        space = Reserve::SIZE,
    )]
    pub reserve: Account<'info, Reserve>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [ reserve.authority.key().as_ref() ],
        bump,
    )]
    pub reserve: Account<'info, Reserve>,
    /// CHECK:
    #[account(mut)]
    pub source: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [ reserve.authority.key().as_ref() ],
        bump,
    )]
    pub reserve: Account<'info, Reserve>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [ authority.key().as_ref() ],
        bump,
    )]
    pub reserve: Account<'info, Reserve>,
    /// CHECK:
    #[account(mut)]
    pub authority: Signer<'info>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("You cannot withdraw more than the limited amount (10 lamports in testing).")]
    OverWithdrawLimit,
}
