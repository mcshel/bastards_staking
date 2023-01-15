use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::{program::invoke_signed, sysvar};
use mpl_token_metadata as metaplex;

use crate::states::*;
use crate::utils::*;
use crate::errors::StakingErrors;



/*
 *  Stake-delegate an NFT
 */


#[derive(Accounts)]
pub struct StakeMine<'info> {

    // Mine account in which to stake the NFT
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Box<Account<'info, Mine>>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Box<Account<'info, Character>>,
    
    // Whitelist account to be used for whitelist proof
    pub whitelist: Box<Account<'info, Whitelist>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // Associated token account for paying the fee
    #[account(
        mut,
        constraint = mine.fee_proceeds == fee_proceeds.key(),
    )]
    pub fee_proceeds: Account<'info, TokenAccount>,
    
    // Associated token account for paying the fee
    #[account(
        mut,
        associated_token::mint = mine.fee_mint,
        associated_token::authority = user,
    )]
    pub fee_ata: Account<'info, TokenAccount>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn stake(ctx: Context<StakeMine>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let character = &mut ctx.accounts.character;
    let whitelist = &ctx.accounts.whitelist;
    
    require!(
        !mine.locked,
        StakingErrors::StakingPoolLocked
    );
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    require!(
        character.staked == 0,
        StakingErrors::AlreadyStaked
    );
    
    require!(
        whitelist.faction == 1,
        StakingErrors::InvalidFaction
    );
    
    require!(
        character.timestamp + mine.cooldown < clock.unix_timestamp,
        StakingErrors::Cooldown
    );
    
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    mine.add_character()?;
    
    character.stake(1, mine.accrued_rewards, clock.unix_timestamp)?;
    
    if mine.fee > 0 {
        // Transfer the Mine staking fee
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.fee_ata.to_account_info(),
                    to: ctx.accounts.fee_proceeds.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            mine.fee,
        )?;
    }
    
    // Add the Mine account as delegate to user's NFT ATA
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                to: ctx.accounts.nft_ata.to_account_info(),
                delegate: mine.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1,
    )?;
    
    // Freeze the user's NFT ATA
    invoke_signed(
        &metaplex::instruction::freeze_delegated_account(
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
    
    Ok(())
}



/*
 *  Claim mining reward
 */


#[derive(Accounts)]
pub struct ClaimMine<'info> {

    // Mine staking pool account
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Box<Account<'info, Mine>>,
    
    // Loot staking pool account
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Box<Account<'info, Loot>>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Box<Account<'info, Character>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Box<Account<'info, TokenAccount>>,
    
    // Mint account of the NFT
    pub nft_mint: Box<Account<'info, Mint>>,
    
    // Associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key(),
        constraint = mine.mint == reward_mint.key(),
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECKED: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    recent_slothashes: UncheckedAccount<'info>,
}


pub fn claim(ctx: Context<ClaimMine>) -> Result<()> {
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    require!(
        character.staked == 1,
        StakingErrors::NotStaked
    );
    
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    let slothash = &ctx.accounts.recent_slothashes.data.borrow().to_vec();
    let (accrued_reward, secured_reward, loot_funding) = compute_mine_rewards(character, mine, slothash, clock.unix_timestamp)?;
    
    character.claim(mine.accrued_rewards, accrued_reward, secured_reward)?;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.add_funds(clock.unix_timestamp, loot_funding)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
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
        secured_reward,
    )?;
    
    Ok(())
}



/*
 *  Unstake-delegate an NFT
 */


#[derive(Accounts)]
pub struct UnstakeMine<'info> {

    // Mine staking pool account
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Box<Account<'info, Mine>>,
    
    // Loot staking pool account
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Box<Account<'info, Loot>>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Box<Account<'info, Character>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Box<Account<'info, TokenAccount>>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Box<Account<'info, Mint>>,
    
    // Associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key(),
        constraint = mine.mint == reward_mint.key(),
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECKED: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    recent_slothashes: UncheckedAccount<'info>,
}


pub fn unstake(ctx: Context<UnstakeMine>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    require!(
        character.staked == 1,
        StakingErrors::NotStaked
    );
    
    require!(
        character.timestamp + mine.warmup < clock.unix_timestamp,
        StakingErrors::Warmup
    );
    
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    let slothash = &ctx.accounts.recent_slothashes.data.borrow().to_vec();
    let (accrued_reward, secured_reward, loot_funding) = compute_mine_rewards(character, mine, slothash, clock.unix_timestamp)?;
    
    character.unstake(accrued_reward, secured_reward, clock.unix_timestamp)?;
    
    mine.remove_character()?;
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.add_funds(clock.unix_timestamp, loot_funding)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
    
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
        secured_reward,
    )?;
    
    Ok(())
}



/*
 *  Unstake-delegate an NFT
 */


#[derive(Accounts)]
pub struct ForceUnstakeMine<'info> {

    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,

    // Mine staking pool account
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Box<Account<'info, Mine>>,
    
    // Loot staking pool account
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Box<Account<'info, Loot>>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Box<Account<'info, Character>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Box<Account<'info, TokenAccount>>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Box<Account<'info, Mint>>,
    
    // Associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key(),
        constraint = mine.mint == reward_mint.key(),
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
    // User account that holds the NFT
    ///CHECKED: implicity checked against the NFT holder
    #[account(mut)]
    pub user: UncheckedAccount<'info>,
    
    // Admin account
    #[account(
        mut,
        constraint = admin_settings.admin == authority.key()
    )]
    pub authority: Signer<'info>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECKED: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    recent_slothashes: UncheckedAccount<'info>,
}


pub fn force_unstake(ctx: Context<ForceUnstakeMine>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        StakingErrors::NotHolder
    );
    
    require!(
        character.staked == 1,
        StakingErrors::NotStaked
    );
    
    require!(
        character.timestamp + mine.warmup < clock.unix_timestamp,
        StakingErrors::Warmup
    );
    
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    let slothash = &ctx.accounts.recent_slothashes.data.borrow().to_vec();
    let (accrued_reward, secured_reward, loot_funding) = compute_mine_rewards(character, mine, slothash, clock.unix_timestamp)?;
    
    character.unstake(accrued_reward, secured_reward, clock.unix_timestamp)?;
    
    mine.remove_character()?;
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.add_funds(clock.unix_timestamp, loot_funding)?;
    loot.recompute_rate(clock.unix_timestamp)?;
    
    
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
    
    /*
    // Remove the Mine account as delegate from user's NFT ATA
    token::revoke(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Revoke {
                source: ctx.accounts.nft_ata.to_account_info(),
                authority: mine.to_account_info(),
            },
        )
    )?;
    */
    
    
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
        secured_reward,
    )?;
    
    Ok(())
}



/*
 *  Utility function for computing the mining rewards
 */

 
fn compute_mine_rewards(character: &mut Character, mine: &mut Mine, slothash: &Vec<u8>, timestamp: i64) -> Result<(u64, u64, u64)> {
    
    // Compute accrued reward
    let accrued_reward = u64::try_from(
        mine.accrued_rewards.checked_sub(character.staked_peg).ok_or(StakingErrors::InvalidComputation)?
    ).unwrap();
    
    
    // Compute secured reward
    let seed = get_rand_u128(slothash, timestamp)?;
    
    let range = mine.reward_precision.checked_sub(
        mine.reward_min_fraction.checked_mul(2).ok_or(StakingErrors::InvalidComputation)?
    ).ok_or(StakingErrors::InvalidComputation)?;
    
    let secured_fraction = seed.checked_rem(
        range as u128
    ).ok_or(StakingErrors::InvalidComputation)?.checked_add(
        mine.reward_min_fraction as u128
    ).unwrap();
    
    let secured_reward = u64::try_from(
        secured_fraction.checked_mul(
            accrued_reward as u128
        ).ok_or(StakingErrors::InvalidComputation)?.checked_div(
            mine.reward_precision as u128
        ).ok_or(StakingErrors::InvalidComputation)?.checked_mul(
            mine.boost as u128
        ).ok_or(StakingErrors::InvalidComputation)?.checked_div(
            mine.reward_precision as u128
        ).ok_or(StakingErrors::InvalidComputation)?
    ).unwrap();
    
    
    // Compute loot funding
    let loot_funding = accrued_reward.checked_div(2).ok_or(StakingErrors::InvalidComputation)?;
    
    msg!("Accrued reward: {}", accrued_reward);
    msg!("Secured fraction: {}", secured_fraction);
    msg!("Secured reward: {}", secured_reward);
    Ok((accrued_reward, secured_reward, loot_funding))
}
