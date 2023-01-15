use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Mine {

    // Bump used in generating the Mine account
    pub bump: u8,

    // Account of the mint that is being mined
    pub mint: Pubkey,
    
    // Number of staked NFTs
    pub staked_characters: u16,
    
    // Comulative accrued rewards per mining point
    pub accrued_rewards: u128,
    
    // Timestamp the of the last comulative accrued reward update
    pub accrued_timestamp: i64,
    
    // Lock controling if NFTs can be staked into the pool
    pub locked: bool,
    
    // Fee mint
    pub fee_mint: Pubkey,
    
    // Fee account
    pub fee_proceeds: Pubkey,
    
    // Fee for staking in the Loot pool
    pub fee: u64,
    
    // Mining reward rate in units of Reward Tokens / mining point / s
    pub rate: u64,
    
    // Warmup period for staking in Mine pool
    pub warmup: i64,
    
    // Cooldown period for re-staking in Mine pool
    pub cooldown: i64,
    
    // Boost factor for the Mine pool rewards
    pub boost: u16,
    
    // Min reward fraction
    pub reward_min_fraction: u16,
    
    // Reward precision
    pub reward_precision: u16,
    
}

impl Mine {

    pub fn initialize(&mut self, bump: u8, mint: &Pubkey, fee_mint: &Pubkey, fee_proceeds: &Pubkey) -> Result<()> {
    
        self.bump = bump;
        self.mint = *mint;
        self.fee_mint = *fee_mint;
        self.fee_proceeds = *fee_proceeds;
        self.staked_characters = 0;
        self.accrued_rewards = 0;
        self.accrued_timestamp = 0;
        self.locked = true;
        self.fee = 0;
        self.rate = 0;
        self.warmup = 0;
        self.cooldown = 0;
        self.boost = 1000;
        self.reward_min_fraction = 250;
        self.reward_precision = 1000;
        
        Ok(())
    }
    

    pub fn update_accrued_rewards(&mut self, timestamp: i64) -> Result<()> {
        
        let timestamp_delta_u128 = u128::try_from(
            timestamp.checked_sub(self.accrued_timestamp).ok_or(StakingErrors::InvalidComputation)?
        ).unwrap();
        let rate_u128 = self.rate as u128;
        let newly_accrued_rewards = rate_u128.checked_mul(timestamp_delta_u128).ok_or(StakingErrors::InvalidComputation)?;
        
        self.accrued_rewards = self.accrued_rewards.checked_add(newly_accrued_rewards).ok_or(StakingErrors::InvalidComputation)?;
        self.accrued_timestamp = timestamp;
        
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
