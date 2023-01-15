use anchor_lang::prelude::*;

use crate::states::*;
use crate::errors::StakingErrors;



/*
 *  Set the Loot pool parameters
 */

 
#[derive(Accounts)]
pub struct SetLootParameters<'info> {

    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,

    // Address of the Loot pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Admin account
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key()
    )]
    pub authority: Signer<'info>,
}



pub fn set_parameters(ctx: Context<SetLootParameters>, locked: bool, fee: u64, duration: u64, warmup: i64, cooldown: i64, boost: u16, reward_probability: u16, reward_precision: u16) -> Result<()> {
    
    require!(
        reward_probability < reward_precision,
        StakingErrors::InvalidParameters
    );
    
    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    
    loot.locked = locked;
    loot.fee = fee;
    loot.duration = duration;
    loot.warmup = warmup;
    loot.cooldown = cooldown;
    loot.boost = boost;
    loot.reward_probability = reward_probability;
    loot.reward_precision = reward_precision;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
    Ok(())
}



/*
 *  Fund the Loot pool
 */

 
#[derive(Accounts)]
pub struct FundLoot<'info> {

    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,

    // Address of the Loot pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Admin account
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key()
    )]
    pub authority: Signer<'info>,
}



pub fn fund(ctx: Context<FundLoot>, amount: u64) -> Result<()> {
    
    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.add_funds(clock.unix_timestamp, amount)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
    Ok(())
}



/*
 *  Update Loot staking pool's accrued rewards
 */
 
 
#[derive(Accounts)]
pub struct UpdateLoot<'info> {

    // Address of the Loot staking pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
}



pub fn update(ctx: Context<UpdateLoot>) -> Result<()> {

    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
    Ok(())
}
