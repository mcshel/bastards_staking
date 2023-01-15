use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Character { 
    
    // Bump used in generating the Character account
    pub bump: u8,
    
    // Comulative amount of accrued rewards
    pub rewards_accrued: u128,
    
    // Comulative amount of secured rewards
    pub rewards_secured: u128,
    
    // NFT staking status: 0 unstaked, 1 staked in mine, 2 staked in loot
    pub staked: u8,
    
    // Staking pool's commualtive accrued rewards per mining point at the moment of staking
    pub staked_peg: u128,
    
    // Timestamp when the NFT was staked
    pub timestamp: i64,
}


impl Character {
    
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
    
        self.bump = bump;
        
        self.rewards_accrued = 0;
        self.rewards_secured = 0;
        
        self.staked = 0;
        self.staked_peg = 0;
        self.timestamp = 0;
        
        Ok(())
    }
    
    
    pub fn stake(&mut self, pool: u8, peg: u128, timestamp: i64) -> Result<()> {
        
        self.staked = pool;
        self.staked_peg = peg;
        self.timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn claim(&mut self, peg: u128, accrued_reward: u64, secured_reward: u64) -> Result<()> {
        
        self.staked_peg = peg;
        self.rewards_accrued = self.rewards_accrued.checked_add(accrued_reward as u128).ok_or(StakingErrors::InvalidComputation)?;
        self.rewards_secured = self.rewards_secured.checked_add(secured_reward as u128).ok_or(StakingErrors::InvalidComputation)?;
        
        return Ok(())
    }
    
    
    pub fn unstake(&mut self, accrued_reward: u64, secured_reward: u64, timestamp: i64) -> Result<()> {
        
        self.staked = 0;
        self.timestamp = timestamp;
        
        self.claim(0, accrued_reward, secured_reward)?;
        
        return Ok(())
    }
}
