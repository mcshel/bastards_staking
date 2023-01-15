use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Loot {

    // Bump used in generating the Loot account
    pub bump: u8,
    
    // Fee mint
    pub fee_mint: Pubkey,
    
    // Fee account
    pub fee_proceeds: Pubkey,
    
    // Total reward fund
    pub fund: u128,
    
    // Mining reward rate in units of Reward Tokens / mining point / s
    pub rate: u64,
    
    // Number of staked NFTs
    pub staked_characters: u16,
    
    // Comulative accrued rewards per mining point
    pub accrued_rewards: u128,
    
    // Timestamp the of the last comulative accrued reward update
    pub accrued_timestamp: i64,
    
    // Timestamp of the last change in pool's fund
    pub funding_timestamp: i64,
    
    // Lock controling if NFTs can be staked into the pool
    pub locked: bool,
    
    // Fee for staking in the Loot pool
    pub fee: u64,
    
    // Duration ower which the reward fund will be distributed in units of s
    pub duration: u64,
    
    // Warmup period for staking in Loot pool
    pub warmup: i64,
    
    // Cooldown period for re-staking in Loot pool
    pub cooldown: i64,
    
    // Boost factor for the Loot pool rewards
    pub boost: u16,
    
    // Probability to successfully claim the reward
    pub reward_probability: u16,
    
    // Reward precision
    pub reward_precision: u16,
}

impl Loot {

    pub fn initialize(&mut self, bump: u8, fee_mint: &Pubkey, fee_proceeds: &Pubkey) -> Result<()> {
    
        self.bump = bump;
        self.fee_mint = *fee_mint;
        self.fee_proceeds = *fee_proceeds;
        self.locked = true;
        self.fund = 0;
        self.rate = 0;
        self.staked_characters = 0;
        self.accrued_rewards = 0;
        self.accrued_timestamp = 0;
        self.funding_timestamp = 0;
        self.fee = 0;
        self.duration = 0;
        self.warmup = 0;
        self.cooldown = 0;
        self.boost = 1000;
        self.reward_probability = 800;
        self.reward_precision = 1000;
        
        Ok(())
    }
    

    pub fn update_accrued_rewards(&mut self, timestamp: i64) -> Result<()> {
        
        let timestamp_delta_u128 = u128::try_from(
            timestamp.checked_sub(self.accrued_timestamp).ok_or(StakingErrors::InvalidComputation)?
        ).unwrap();
        
        let mut newly_accrued_rewards =  0_u128;
        if self.staked_characters > 0 {
            let rate_u128 = self.rate as u128;
            newly_accrued_rewards = rate_u128.checked_mul(timestamp_delta_u128).ok_or(StakingErrors::InvalidComputation)?;
            
            let available_rewards = self.fund.checked_div(self.staked_characters as u128).ok_or(StakingErrors::InvalidComputation)?;
            newly_accrued_rewards = std::cmp::min(newly_accrued_rewards, available_rewards);
        }
        
        self.fund = self.fund.checked_sub(
            newly_accrued_rewards.checked_mul(
                self.staked_characters as u128
            ).ok_or(StakingErrors::InvalidComputation)?
        ).ok_or(StakingErrors::InvalidComputation)?;
        
        self.accrued_rewards = self.accrued_rewards.checked_add(newly_accrued_rewards).ok_or(StakingErrors::InvalidComputation)?;
        self.accrued_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn recompute_rate(&mut self, timestamp: i64) -> Result<()> {
        
        let remaining_duration = self.funding_timestamp.checked_add(
            i64::try_from(self.duration).unwrap()
        ).ok_or(StakingErrors::InvalidComputation)?.checked_sub(
            timestamp
        ).ok_or(StakingErrors::InvalidComputation)?;
        
        if remaining_duration > 0 && self.staked_characters > 0 {
            self.rate = u64::try_from(
                self.fund.checked_div(
                    u128::try_from(remaining_duration).unwrap()
                ).ok_or(StakingErrors::InvalidComputation)?.checked_div(
                    self.staked_characters as u128
                ).ok_or(StakingErrors::InvalidComputation)?
            ).unwrap();
        } else {
            self.rate = 0;
        }
        
        Ok(())
    }
    
    
    pub fn add_funds(&mut self, timestamp: i64, amount: u64) -> Result<()> {
        
        self.fund = self.fund.checked_add(amount as u128).ok_or(StakingErrors::InvalidComputation)?;
        self.funding_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn add_character(&mut self) -> Result<()> {
        
        self.staked_characters = self.staked_characters.checked_add(1).ok_or(StakingErrors::InvalidComputation)?;
        
        Ok(())
    }
    
    
    pub fn remove_character(&mut self) -> Result<()> {
        
        self.staked_characters = self.staked_characters.checked_sub(1).ok_or(StakingErrors::InvalidComputation)?;
        
        Ok(())
    }

}
