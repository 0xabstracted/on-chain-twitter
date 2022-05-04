use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    //#[account(mut, has_one = author, seeds = [b"twitter-user".as_ref(), author.key().as_ref()], bump = twitter_user_account.bump)]
    #[account(
        mut, 
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user_account.bump
    )]
    pub twitter_user_account: Account<'info, TwitterUser>,
    //#[account(mut, has_one = author, seeds = [b"tweet-account".as_ref(), author.key().as_ref(),&tweet_account.tweet_number.to_le_bytes()], bump= twitter_user_account.bump, close = author)]
    #[account(
        mut, 
        seeds = [b"tweet-account".as_ref(), author.key().as_ref(), tweet_account.tweet_number.to_le_bytes().as_ref()], 
        bump= tweet_account.bump, 
        close = author
    )]
    //#[account(mut, seeds = [b"tweet-account".as_ref(), author.key().as_ref(),&[twitter_user_account.tweet_count]], bump= twitter_user_account.bump, close = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}


pub fn handler(ctx: Context<DeleteTweet>) -> Result<()>{
    let twitter_user_account:  &mut Account<TwitterUser> = &mut ctx.accounts.twitter_user_account;
    let clock: Clock = Clock::get().unwrap();
    twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}
