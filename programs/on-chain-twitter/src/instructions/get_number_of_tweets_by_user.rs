use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct GetNumberOfTweetsByUser<'info>{
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub author: UncheckedAccount<'info>,
    #[account(
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user_account.bump
    )]
    pub twitter_user_account: Account<'info, TwitterUser>,
}

//pub fn handler(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u64>{
//pub fn handler(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u32>{
pub fn handler(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u8>{
    let twitter_user_account =  &ctx.accounts.twitter_user_account;
Ok(twitter_user_account.tweet_count)
}