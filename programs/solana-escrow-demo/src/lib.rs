use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("B5JyXbr4WwkWsmqG4d51oPW4314CPGx71zvjzBpdHtJw");

#[program]
pub mod escrow_demo {
    use anchor_spl::token::{transfer_checked, TransferChecked};

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64, deposit_amount:u64) -> Result<()> {
        ctx.accounts.escrow.set_inner(Escrow {
            seed,
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.maker.key(),
            mint_b: ctx.accounts.maker.key(),
            receive_amount,
            bump: ctx.bumps.escrow,
        });

        let ctx_accounts = TransferChecked {
            from: ctx.accounts.maker_ata_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.mint_b.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ctx_accounts
        );

        transfer_checked(transfer_ctx,deposit_amount,ctx.accounts.mint_a.decimals)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        Ok(())
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction[seed:u64]]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    mint_a: Account<'info, Mint>,
    mint_b: Account<'info, Mint>,
    #[account(
     associated_token::mint = mint_a,
     associated_token::authority = maker,
  )]
    maker_ata_a: Account<'info, TokenAccount>,
    #[account(
    init,
    payer = maker,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
 )]
    valet: Account<'info, TokenAccount>,
    #[account(
    init,
    payer = maker,
    space = Escrow::INIT_SPACE,
    seeds=[b"escrow",seed.to_le_byte().as_ref(())],
    bump
  )]
    escrow: Account<'info, Escrow>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    seed: u64,
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    receive_amount: u64,
    bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 8 + 32 + 3 + 8 + 1;
}
