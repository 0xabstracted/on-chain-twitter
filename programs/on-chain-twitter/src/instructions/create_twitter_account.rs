use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;


#[derive(Accounts)]
pub struct CreateTwitterAccount <'info>{
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = 8 + TwitterUser::LEN, seeds = [b"twitter-user".as_ref(), author.key().as_ref()], bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTwitterAccount>, username: String) -> Result<()>{
    let twitter_user_account = &mut ctx.accounts.twitter_user_account;
    let clock: Clock = Clock::get().unwrap();
    if username.as_bytes().len() > 64 {
        return Err(ErrorCode::UsernameTooLong.into())
    }
    twitter_user_account.tweet_count  = 0;
    twitter_user_account.author = *ctx.accounts.author.key;
    twitter_user_account.username = username;    
    twitter_user_account.bump = *ctx.bumps.get("twitter_user_account").unwrap();
    twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())

}