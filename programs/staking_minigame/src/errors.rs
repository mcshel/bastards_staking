use anchor_lang::prelude::*;

#[error_code]
pub enum StakingErrors {

    // 6000
    #[msg("Manager account is not on-curve")]
    ManagerOffCurve,
    
    // 6001
    #[msg("Invalid computation")]
    InvalidComputation,
    
    // 6002
    #[msg("The supplied metadata account is not valid")]
    InvalidMetadataAccount,
    
    // 6003
    #[msg("The supplied edition account is not valid")]
    InvalidEditionAccount,
    
    // 6004
    #[msg("The supplied whitelist proof account is not valid")]
    InvalidWhitelistProof,
    
    // 6005
    #[msg("The supplied whitelist type is invalid")]
    InvalidWhitelistType,
    
    // 6006
    #[msg("Invalid parameters")]
    InvalidParameters,
    
    // 6007
    #[msg("The requested staking pool is currently locked")]
    StakingPoolLocked,
    
    // 6008
    #[msg("The user is not the current holder of the NFT")]
    NotHolder,
    
    // 6009
    #[msg("The NFT is already staked")]
    AlreadyStaked,
    
    // 6010
    #[msg("The NFT is not staked")]
    NotStaked,
    
    // 6011
    #[msg("The NFT does not belong to the requested faction")]
    InvalidFaction,
    
    // 6012
    #[msg("The NFT warmup period has not yet elapsed")]
    Warmup,
    
    // 6013
    #[msg("The NFT cooldown period has not yet elapsed")]
    Cooldown,
    
    // 6014
    #[msg("Can not close pool while tokens are still staked")]
    PoolNotEmpty,
} 
