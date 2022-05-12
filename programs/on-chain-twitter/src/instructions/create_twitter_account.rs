use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;


#[derive(Accounts)]
pub struct CreateTwitterAccount <'info>{
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init, 
        payer = author, 
        space = 8 + TwitterUser::LEN, 
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTwitterAccount>, username: String) -> Result<()>{
    let twitter_user = &mut ctx.accounts.twitter_user;
    let clock: Clock = Clock::get().unwrap();
    if username.as_bytes().len() > 64 {
        return Err(ErrorCode::UsernameTooLong.into())
    }
    twitter_user.next_address = Pubkey::default();
    twitter_user.tweet_count  = 0;
    twitter_user.author = *ctx.accounts.author.key;
    twitter_user.username = username;    
    twitter_user.bump = *ctx.bumps.get("twitter_user").unwrap();
    twitter_user.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())

}