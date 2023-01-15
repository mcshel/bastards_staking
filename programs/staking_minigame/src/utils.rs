use arrayref::array_ref;
use anchor_lang::prelude::*;
use mpl_token_metadata as metaplex;

use crate::errors::StakingErrors;



/*
 *  Utility function for checking the validity of edition account
 */


pub fn assert_edition_account(mint: &Pubkey, mint_edition: &AccountInfo) -> Result<()> {
    let metadata_program = metaplex::id();
    
    require_keys_eq!(
        *mint_edition.owner,
        metadata_program,
        StakingErrors::InvalidEditionAccount
    );
    
    let seed = &[b"metadata".as_ref(), metadata_program.as_ref(), mint.as_ref(), b"edition".as_ref()];
    let (edition_account, _bump) = Pubkey::find_program_address(seed, &metadata_program);
    require_keys_eq!(
        edition_account,
        mint_edition.key(),
        StakingErrors::InvalidEditionAccount
    );
    
    Ok(())
}



/*
 *  Utility function for obtaining random u128 integer from recent slothash and timestamp
 */


pub fn get_rand_u128(slothash: &Vec<u8>, timestamp: i64) -> Result<u128> {
    let subhash = array_ref![slothash, 3, 16];
    let subhash_value = u128::from_le_bytes(*subhash);
    let timestamp_u128 = u128::try_from(timestamp).unwrap();
    let value = subhash_value.wrapping_add(timestamp_u128);
    
    Ok(value)
}
