use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::*;
use crate::errors::StakingErrors;



/*
 *  Set the Mine pool parameters
 */

 
#[derive(Accounts)]
pub struct SetMineParameters<'info> {

    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Admin account
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key()
    )]
    pub authority: Signer<'info>,
}



pub fn set_parameters(ctx: Context<SetMineParameters>, locked: bool, fee: u64, rate: u64, warmup: i64, cooldown: i64, boost: u16, reward_min_fraction: u16, reward_precision: u16) -> Result<()> {
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    
    require!(
        reward_precision > reward_min_fraction.checked_mul(2).ok_or(StakingErrors::InvalidComputation)?,
        StakingErrors::InvalidParameters
    );
    
    mine.locked = locked;
    mine.fee = fee;
    mine.rate = rate;
    mine.warmup = warmup;
    mine.cooldown = cooldown;
    mine.boost = boost;
    mine.reward_min_fraction = reward_min_fraction;
    mine.reward_precision = reward_precision;
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    Ok(())
}


/*
 *  Update mine's accrued rewards
 */
 
 
#[derive(Accounts)]
pub struct UpdateMine<'info> {

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
}



pub fn update(ctx: Context<UpdateMine>) -> Result<()> {

    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    Ok(())
}



/*
 *  Mint rewards
 */
 
 
#[derive(Accounts)]
pub struct MintMineRewards<'info> {

    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Target associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = authority,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key(),
        constraint = mine.mint == reward_mint.key(),
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
    // Admin account
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key()
    )]
    pub authority: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}



pub fn mint(ctx: Context<MintMineRewards>, amount: u64) -> Result<()> {

    let mine = &ctx.accounts.mine;

    // Mint the reward tokens to user's ATA
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.reward_ata.to_account_info(),
                authority: mine.to_account_info(),
            },
            &[&[b"mine".as_ref(), &[mine.bump]]],
        ),
        amount,
    )?;
    
    Ok(())
}



