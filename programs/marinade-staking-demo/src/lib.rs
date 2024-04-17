use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use marinade::cpi::accounts::{Deposit as MarinadeDeposit, LiquidUnstake as MarinadeLiquidUnstake};
use marinade::cpi::{deposit as marinade_deposit, liquid_unstake as marinade_liquid_unstake};
use marinade::program::MarinadeFinance;
use marinade::State as MarinadeState;

declare_id!("DfHJpPQ7BNhGz9LNvohUgqFMDRqfLDeDXk8GKqnryNPT");

#[program]
pub mod marinade_staking_demo {
    use super::*;

    pub fn init<'c: 'info, 'info>(ctx: Context<Init>) -> Result<()> {
        Ok(())
    }

    pub fn deposit<'c: 'info, 'info>(
        ctx: Context<Deposit>,
        amount: u64,
        treasury_bump: u8,
    ) -> Result<()> {
        msg!(
            "mSol will be mint to ATA: {:?} owned by {:?}",
            ctx.accounts.mint_to.key(),
            ctx.accounts.treasury_pda.key()
        );

        let cpi_program = ctx.accounts.marinade_program.to_account_info();
        let cpi_accounts = MarinadeDeposit {
            state: ctx.accounts.marinade_state.to_account_info(),
            msol_mint: ctx.accounts.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: ctx.accounts.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: ctx.accounts.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: ctx.accounts.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: ctx.accounts.reserve_pda.to_account_info(),
            transfer_from: ctx.accounts.signer.to_account_info(),
            mint_to: ctx.accounts.mint_to.to_account_info(),
            msol_mint_authority: ctx.accounts.msol_mint_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let seeds = &["treasury".as_bytes(), &[treasury_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let _ = marinade_deposit(cpi_ctx, amount);
        Ok(())
    }

    pub fn unstake<'c: 'info, 'info>(_ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub marinade_state: Account<'info, MarinadeState>,

    /// CHECK: skip
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,

    #[account(mut)]
    pub msol_mint: Account<'info, Mint>,

    /// CHECK: skip
    #[account(mut)]
    pub msol_mint_authority: AccountInfo<'info>,

    /// CHECK: skip
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,

    /// CHECK: skip
    #[account(mut)]
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,

    /// CHECK: skip
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = msol_mint,
        associated_token::authority = treasury_pda,
        associated_token::token_program = token_program
    )]
    pub mint_to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_pda: Account<'info, Treasury>,

    pub marinade_program: Program<'info, MarinadeFinance>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init_if_needed, seeds = [b"treasury".as_ref()], bump, payer = signer, space = 8)]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Treasury {}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub marinade_program: Program<'info, MarinadeFinance>,
}
