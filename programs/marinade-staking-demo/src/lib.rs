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

    pub fn deposit<'c: 'info, 'info>(_ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Depositing {} tokens", amount);
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

    pub marinade_program: Program<'info, MarinadeFinance>,
    // pub drift_program: Program<'info, Drift>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // pub marinade_program: Program<'info, MarinadeFinance>,
}
