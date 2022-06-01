use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("AVD6temvivgz6p1S9xaSNmnLnfPdsQfwNVXyVEGCSr44");

#[program]
pub mod solana_backend {
    use super::*;

    pub fn transfer_in(ctx: Context<TransferIn>, amount: u64) -> Result<()> {
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                },
            );
            transfer(cpi_ctx, amount)?;
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
    /// CHECK: not now
    mint: AccountInfo<'info>,
    owner: Signer<'info>,
    system_program: Program<'info, System>
}
