use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::transfer;

#[program]
mod token_contract {
    use super::*;

    #[state]
    pub struct TokenContract {
        pub mint: Pubkey,
        pub authority: Pubkey,
    }

    impl TokenContract {
        pub fn new(ctx: Context<Initialize>, decimals: u8) -> Result<Self> {
            let mint = Mint::new(&ctx.accounts.system_program, decimals)?;
            Ok(Self {
                mint: mint.key(),
                authority: *ctx.accounts.authority.key,
            })
        }

        pub fn mint_token(&self, ctx: Context<MintToken>, amount: u64) -> ProgramResult {
            let cpi_accounts = Transfer {
                from: ctx.accounts.system_program.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::mint_to(cpi_ctx, amount)?;

            Ok(())
        }

        pub fn transfer_token(&self, ctx: Context<TransferToken>, amount: u64) -> ProgramResult {
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;

            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    authority: Signer<'info>,
    #[account(mut)]
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
}