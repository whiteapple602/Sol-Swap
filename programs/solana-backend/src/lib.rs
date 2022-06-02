use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer};
use anchor_lang::system_program;
use anchor_lang::system_program::{transfer};
declare_id!("AVD6temvivgz6p1S9xaSNmnLnfPdsQfwNVXyVEGCSr44");

#[program]
pub mod solana_backend {
    use super::*;

    pub fn transfer_in(ctx: Context<TransferIn>, amount: u64) -> Result<()> {
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                },
            );
            transfer(cpi_ctx, amount)?;
        }

        Ok(())
    }

    pub fn transfer_out(ctx: Context<TransferOut>) -> Result<()> {
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            );
            token::transfer(cpi_ctx, 1)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferIn<'info> {
    #[account(mut)]
    from: Account<'info, TokenAccount>,
    #[account(mut)]
    to: Account<'info, TokenAccount>,
    mint: AccountInfo<'info>,
    owner: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct TransferOut<'info> {
    #[account(mut)]
    from: Account<'info, TokenAccount>,
    #[account(mut)]
    to: Account<'info, TokenAccount>,
    mint: AccountInfo<'info>,
    owner: Signer<'info>,
    token_program: Program<'info, Token>,
}
