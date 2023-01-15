use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;
pub mod utils;
pub mod errors;

declare_id!("3hdVVxxUCM7oGVxRYphvRaNziwmDWy4KARy9AFNmXeMS");


#[program]
pub mod staking_minigame {

    use super::*;
    
    
    // ----- Staking program config functions -----
    
    pub fn init_admin(ctx: Context<InitAdmin>, admin: Pubkey) -> Result<()> {
        instructions::program_config::init_admin(ctx, admin)?;
        Ok(())
    }
    
    
    pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {
        instructions::program_config::set_admin(ctx, admin)?;
        Ok(())
    }
    
    
    pub fn init_pools(ctx: Context<InitPools>) -> Result<()> {
        instructions::program_config::init_pools(ctx)?;
        Ok(())
    }
    
    
    pub fn close_pools(ctx: Context<ClosePools>) -> Result<()> {
        instructions::program_config::close_pools(ctx)?;
        Ok(())
    }
    
    
    pub fn add_whitelist(ctx: Context<AddWhitelist>, reference_account: Pubkey, whitelist_type: u8, faction: u8) -> Result<()> {
        instructions::program_config::add_whitelist(ctx, reference_account, whitelist_type, faction)?;
        Ok(())
    }
    
    
    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, reference_account: Pubkey) -> Result<()> {
        instructions::program_config::remove_whitelist(ctx, reference_account)?;
        Ok(())
    }
    
    
    pub fn add_character(ctx: Context<AddCharacter>) -> Result<()> {
        instructions::program_config::add_character(ctx)?;
        Ok(())
    }
    
    pub fn remove_character(ctx: Context<RemoveCharacter>) -> Result<()> {
        instructions::program_config::remove_character(ctx)?;
        Ok(())
    }
    
    
    // ----- Mine config functions ----
    
    pub fn set_mine_parameters(ctx: Context<SetMineParameters>, locked: bool, fee: u64, rate: u64, warmup: i64, cooldown: i64, boost: u16, reward_min_fraction: u16, reward_precision: u16) -> Result<()> {
        instructions::mine_config::set_parameters(ctx, locked, fee, rate, warmup, cooldown, boost, reward_min_fraction, reward_precision)?;
        Ok(())
    }
    
    
    pub fn update_mine(ctx: Context<UpdateMine>) -> Result<()> {
        instructions::mine_config::update(ctx)?;
        Ok(())
    }
    
    
    pub fn mint_mine_rewards(ctx: Context<MintMineRewards>, amount: u64) -> Result<()> {
        instructions::mine_config::mint(ctx, amount)?;
        Ok(())
    }
    
    
    // ----- Loot config functions -----
    
    pub fn set_loot_parameters(ctx: Context<SetLootParameters>, locked: bool, fee: u64, duration: u64, warmup: i64, cooldown: i64, boost: u16, reward_probability: u16, reward_precision: u16) -> Result<()> {
        instructions::loot_config::set_parameters(ctx, locked, fee, duration, warmup, cooldown, boost, reward_probability, reward_precision)?;
        Ok(())
    }
    
    
    pub fn fund_loot(ctx: Context<FundLoot>, amount: u64) -> Result<()> {
        instructions::loot_config::fund(ctx, amount)?;
        Ok(())
    }
    
    
    pub fn update_loot(ctx: Context<UpdateLoot>) -> Result<()> {
        instructions::loot_config::update(ctx)?;
        Ok(())
    }
    
    
    // ----- Mine staking functions -----
    
    pub fn stake_mine(ctx: Context<StakeMine>) -> Result<()> {
        instructions::mine_staking::stake(ctx)?;
        Ok(())
    }
    
    
    pub fn claim_mine(ctx: Context<ClaimMine>) -> Result<()> {
        instructions::mine_staking::claim(ctx)?;
        Ok(())
    }
    
    
    pub fn unstake_mine(ctx: Context<UnstakeMine>) -> Result<()> {
        instructions::mine_staking::unstake(ctx)?;
        Ok(())
    }
    
    
    pub fn force_unstake_mine(ctx: Context<ForceUnstakeMine>) -> Result<()> {
        instructions::mine_staking::force_unstake(ctx)?;
        Ok(())
    }
    
    
    // ----- Loot staking functions -----
    
    pub fn stake_loot(ctx: Context<StakeLoot>) -> Result<()> {
        instructions::loot_staking::stake(ctx)?;
        Ok(())
    }
    
    
    pub fn claim_loot(ctx: Context<ClaimLoot>) -> Result<()> {
        instructions::loot_staking::claim(ctx)?;
        Ok(())
    }
    
    
    pub fn unstake_loot(ctx: Context<UnstakeLoot>) -> Result<()> {
        instructions::loot_staking::unstake(ctx)?;
        Ok(())
    }
    
    
    pub fn force_unstake_loot(ctx: Context<ForceUnstakeLoot>) -> Result<()> {
        instructions::loot_staking::force_unstake(ctx)?;
        Ok(())
    }
}
