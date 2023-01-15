use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;
use spl_token::instruction::AuthorityType;
use mpl_token_metadata as metaplex;

use crate::program::StakingMinigame;
use crate::states::*;
use crate::utils::*;
use crate::errors::StakingErrors;



/*
 * Initialize program's AddminSettings account and set a program admin
 */


#[derive(Accounts)]
pub struct InitAdmin<'info> {

    // AdminSettings account
    #[account(
        init,
        seeds = [b"admin".as_ref()], 
        bump, 
        payer = authority,
        space = 8 + std::mem::size_of::<AdminSettings>(),
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // StakingMinigame program
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, StakingMinigame>,
    
    // StakingMinigame program data
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    
    // Authority for creating the AdminSettings account -> upgrade authority of the StakingMinigame program
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn init_admin(ctx: Context<InitAdmin>, admin: Pubkey) -> Result<()> {

    let admin_settings = &mut ctx.accounts.admin_settings;
    admin_settings.bump = *ctx.bumps.get("admin_settings").unwrap();
    admin_settings.admin = admin;
    
    Ok(())
}



/*
 *  Set admin by updating the AdminSettings account
 */


#[derive(Accounts)]
pub struct SetAdmin<'info> {

    // AdminSettings account
    #[account(
        mut,
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // StakingMinigame program
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, StakingMinigame>,
    
    // StakingMinigame program data
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    
    // Authority for updating the AdminSettings account -> upgrade authority of the StakingMinigame program
    #[account(mut)]
    pub authority: Signer<'info>,
} 


pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {

    let admin_settings = &mut ctx.accounts.admin_settings;
    admin_settings.admin = admin;
    
    Ok(())
}



/*
 *  Initialize the Mine and Loot staking pools
 *  There is a hard limit of 1 Mine and 1 Loot pool per StakingMinigame smart contract. The init() function 
 *  can only be called by the the smart contract administrator.
 */
 

#[derive(Accounts)]
pub struct InitPools<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Address of the Mine
    #[account(
        init,
        payer = authority,
        seeds = [b"mine".as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Mine>()
    )]
    pub mine: Account<'info, Mine>,
    
    // Address of the Loot
    #[account(
        init,
        payer = authority,
        seeds = [b"loot".as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Loot>()
    )]
    pub loot: Account<'info, Loot>,
    
    // Address of the reward mint
    #[account(
        mint::authority = mine.key()
    )]
    pub mint: Account<'info, Mint>,
    
    // Address of the fee mint
    pub fee_mint: Account<'info, Mint>,
    
    // Associated token account of fee proceeds
    #[account(
        mut,
        associated_token::mint = fee_mint,
        associated_token::authority = authority,
    )]
    pub fee_proceeds: Account<'info, TokenAccount>,
    
    // Authority for creating the Mine -> upgrade authority of the StakingMinigame program
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn init_pools(ctx: Context<InitPools>) -> Result<()> {
    
    let mine = &mut ctx.accounts.mine;
    mine.initialize(*ctx.bumps.get("mine").unwrap(), &ctx.accounts.mint.key(), &ctx.accounts.fee_mint.key(), &ctx.accounts.fee_proceeds.key())?;
    
    let loot = &mut ctx.accounts.loot;
    loot.initialize(*ctx.bumps.get("loot").unwrap(), &ctx.accounts.fee_mint.key(), &ctx.accounts.fee_proceeds.key())?;
        
    Ok(())
}



/*
 *  Close the Mine and Loot staking pools
 */
 
 
#[derive(Accounts)]
pub struct ClosePools<'info> {
    
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
        close = authority,
    )]
    pub mine: Account<'info, Mine>,
    
    // Address of the Loot
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
        close = authority,
    )]
    pub loot: Account<'info, Loot>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key()
    )]
    pub mint: Account<'info, Mint>,
    
    // Authority for creating the Mine -> upgrade authority of the StakingMinigame program
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
}


pub fn close_pools(ctx: Context<ClosePools>) -> Result<()> {

    let mine = &ctx.accounts.mine;
    let loot = &ctx.accounts.loot;
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    
    require!(
        mine.staked_characters == 0,
        StakingErrors::PoolNotEmpty
    );
    
    require!(
        loot.staked_characters == 0,
        StakingErrors::PoolNotEmpty
    );

    token::set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: mine.to_account_info(),
                account_or_mint: mint.to_account_info(),
            },
            &[&[b"mine".as_ref(), &[mine.bump]]],
        ),
        AuthorityType::MintTokens,
        Some(authority.key())
    )?;
    
    Ok(())
}



/*
 *  Add reference account to whitelist
 */


#[derive(Accounts)]
#[instruction(_reference_account: Pubkey)]
pub struct AddWhitelist<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Whitelist account
    #[account(
        init,
        payer = authority,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Whitelist>(),
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Staking program admin defined in AdminSettings
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn add_whitelist(ctx: Context<AddWhitelist>, _reference_account: Pubkey, whitelist_type: u8, faction: u8) -> Result<()> {
    
    //TODO Add sanity checks for the reference_account
    
    require!(
        whitelist_type < 2,
        StakingErrors::InvalidWhitelistType
    );
    
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.bump = *ctx.bumps.get("whitelist").unwrap();
    whitelist.whitelist_type = whitelist_type;
    whitelist.faction = faction;
    
    Ok(())
}



/*
 *  Remove reference account from whitelist
 */


#[derive(Accounts)]
#[instruction(_reference_account: Pubkey)]
pub struct RemoveWhitelist<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Whitelist account
    #[account(
        mut,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        close = authority,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Staking program admin defined in AdminSettings
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key(),
    )]
    pub authority: Signer<'info>,
}


pub fn remove_whitelist(_ctx: Context<RemoveWhitelist>, _reference_account: Pubkey) -> Result<()> {

    Ok(())
}



/*
 *  Create NFT's Character account
 */
 

#[derive(Accounts)]
pub struct AddCharacter<'info> {
    
    // Character account of the NFT
    #[account(
        init,
        payer = user,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Character>(),
    )]
    pub character: Account<'info, Character>,
    
    // Whitelist account to be used for whitelist proof
    whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
}


pub fn add_character(ctx: Context<AddCharacter>) -> Result<()> {

    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    let whitelist = &ctx.accounts.whitelist;
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    
    let character = &mut ctx.accounts.character;
    character.initialize(*ctx.bumps.get("character").unwrap())?;
    
    Ok(())
}



/*
 *  Remove NFT's Character account
 */
 

#[derive(Accounts)]
pub struct RemoveCharacter<'info> {

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Address of the Loot
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
        close = user,
    )]
    pub character: Account<'info, Character>,
    
    // Whitelist account to be used for whitelist proof
    whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
}


pub fn remove_character(ctx: Context<RemoveCharacter>) -> Result<()> {

    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    let whitelist = &ctx.accounts.whitelist;
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    
    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let character = &ctx.accounts.character;
    
    if character.staked == 1 {
    
        let mine = &mut ctx.accounts.mine;
        if mine.staked_characters > 0 {
            let _accrued_rewards = mine.remove_character();
        }
    
        // Thaw the user's NFT ATA
        invoke_signed(
            &metaplex::instruction::thaw_delegated_account(
                ctx.accounts.token_metadata_program.key(),
                mine.key(),
                ctx.accounts.nft_ata.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.token_metadata_program.to_account_info(),
                mine.to_account_info(),
                ctx.accounts.nft_ata.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
            ],
            &[&[b"mine".as_ref(), &[mine.bump]]],
        )?;
        
        // Remove the Mine account as delegate from user's NFT ATA
        token::revoke(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Revoke {
                    source: ctx.accounts.nft_ata.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            )
        )?;
        
    } else if character.staked == 2 {
    
        let loot = &mut ctx.accounts.loot;
        if loot.staked_characters > 0 {
            let _accrued_rewards = loot.remove_character();
        }
    
        // Thaw the user's NFT ATA
        invoke_signed(
            &metaplex::instruction::thaw_delegated_account(
                ctx.accounts.token_metadata_program.key(),
                loot.key(),
                ctx.accounts.nft_ata.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.token_metadata_program.to_account_info(),
                loot.to_account_info(),
                ctx.accounts.nft_ata.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
            ],
            &[&[b"loot".as_ref(), &[loot.bump]]],
        )?;
        
        // Remove the Loot account as delegate from user's NFT ATA
        token::revoke(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Revoke {
                    source: ctx.accounts.nft_ata.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            )
        )?;
    }
    
    Ok(())
}
