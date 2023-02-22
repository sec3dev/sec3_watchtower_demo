# sec3_watchtower_demo
```
==============VULNERABLE: MissingSignerCheck!============
Found a potential vulnerability at line 109, column 8 in src/lib.rs
The account is missing signer check:
 103|        mut,
 104|        seeds = [ reserve.authority.key().as_ref() ],
 105|        bump,
 106|    )]
 107|    pub reserve: Account<'info, Reserve>,
 108|    /// CHECK:
>109|    pub authority: AccountInfo<'info>,
 110|    /// CHECK:
 111|    #[account(mut)]
 112|    pub destination: AccountInfo<'info>,
 113|    pub system_program: Program<'info, System>,
 114|}
 115|#[derive(Accounts)]
 ```
 
 ```
 ==============VULNERABLE: UnvalidatedAccount!============
Found a potential vulnerability at line 37, column 80 in src/lib.rs
The account may not be properly validated and may be untrustful:
 31|            ],
 32|        )?;
 33|        Ok(())
 34|    }
 35|
 36|    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
>37|        let reserve_balance = **ctx.accounts.reserve.to_account_info().lamports.borrow_mut();
 38|        msg!("reserve_balance: {}", reserve_balance);
 39|        msg!("withdraw_amount: {}", amount);
 40|
 41|        if amount > 10 {
 42|            return Err(ErrorCode::OverWithdrawLimit.into());
 43|        }
>>>Stack Trace:
>>>sol.withdraw [src/lib.rs:36]
```
