use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;

#[derive(Accounts)]
pub struct ChangeUserName<'info>{
    pub author: Signer<'info>,
    #[account(mut, seeds = [b"twitter-user".as_ref(), author.key().as_ref()], bump = twitter_user_account.bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
}


pub fn handler(ctx: Context<ChangeUserName>, username: String) -> Result<()>{
    let twitter_user_account = &mut ctx.accounts.twitter_user_account;
    let clock: Clock = Clock::get().unwrap();
    if username.as_bytes().len() > 64 {
        return Err(ErrorCode::NewUsernameTooLong.into())    
    }
    twitter_user_account.username = username;
    twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}