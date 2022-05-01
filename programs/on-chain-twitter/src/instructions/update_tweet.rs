use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;

#[derive(Accounts)]
pub struct UpdateTweet <'info>{
    #[account(mut, has_one = author, seeds = [b"twitter_user".as_ref(), author.key().as_ref()], bump = twitter_user_account.bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    #[account(mut,has_one = author, seeds = [b"tweet_account".as_ref(), author.key().as_ref(),&tweet_account.tweet_number.to_le_bytes()], bump = twitter_user_account.bump)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
    let twitter_user_account:  &mut Account<TwitterUser> = &mut ctx.accounts.twitter_user_account;
    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet_account;
    let clock: Clock = Clock::get().unwrap();
    if topic.as_bytes().len() > 64 {
        return Err(ErrorCode::TopicTooLong.into())
    }
    if content.as_bytes().len() > 512 {
        return Err(ErrorCode::ContentTooLong.into())
    }
    tweet.topic = topic;
    tweet.content = content;
    tweet.tweet_number = twitter_user_account.tweet_count;
    tweet.timestamp = clock.unix_timestamp;
    twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}