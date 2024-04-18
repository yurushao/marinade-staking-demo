use anchor_lang::{prelude::*, system_program};
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

    pub fn init<'c: 'info, 'info>(ctx: Context<Init>, treasury_pda_bump: u8) -> Result<()> {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(500); // Minimum balance to make the account rent-exempt

        let seeds = &["treasury".as_bytes(), &[treasury_pda_bump]];
        let signer_seeds = &[&seeds[..]];

        system_program::create_account(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.treasury_pda.to_account_info().clone(),
                },
                signer_seeds,
            ),
            lamports,
            0, // no data
            &ctx.accounts.system_program.key(),
        )?;

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

        msg!(
            "transfer_from lamports: {:?}",
            ctx.accounts.treasury_pda.lamports()
        );

        require_gte!(ctx.accounts.treasury_pda.lamports(), amount);

        let cpi_program = ctx.accounts.marinade_program.to_account_info();
        let cpi_accounts = MarinadeDeposit {
            state: ctx.accounts.marinade_state.to_account_info(),
            msol_mint: ctx.accounts.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: ctx.accounts.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: ctx.accounts.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: ctx.accounts.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: ctx.accounts.reserve_pda.to_account_info(),
            transfer_from: ctx.accounts.treasury_pda.to_account_info(),
            mint_to: ctx.accounts.mint_to.to_account_info(),
            msol_mint_authority: ctx.accounts.msol_mint_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let seeds = &[b"treasury".as_ref(), &[treasury_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let _ = marinade_deposit(cpi_ctx, amount);
        Ok(())
    }

    pub fn unstake<'c: 'info, 'info>(_ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Treasury {}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(
    //     init_if_needed,
    //     seeds = [b"treasury".as_ref()],
    //     bump,
    //     payer = signer,
    //     owner = system_program.key(),
    //     space = 8
    // )]
    /// CHECK: skip
    #[account(mut)]
    pub treasury_pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
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

    /// CHECK: skip
    #[account(mut)]
    pub treasury_pda: AccountInfo<'info>,

    pub marinade_program: Program<'info, MarinadeFinance>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub marinade_program: Program<'info, MarinadeFinance>,
}
