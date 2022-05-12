use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;

#[derive(Accounts)]
pub struct UpdateTweet <'info>{
    //#[account(mut, has_one = author, seeds = [b"twitter-user".as_ref(), author.key().as_ref()], bump = twitter_user.bump)]
    #[account(
        mut,  
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user.bump
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    //#[account(mut,has_one = author, seeds = [b"tweet-account".as_ref(), author.key().as_ref(),&tweet.tweet_number.to_le_bytes()], bump = twitter_user.bump)]
    #[account(
        mut,
        seeds = [b"tweet-account".as_ref(), author.key().as_ref(), twitter_user.next_address.key().as_ref()], 
        bump = tweet.bump
    )]
    //#[account(mut,seeds = [b"tweet-account".as_ref(), author.key().as_ref(),&[twitter_user.tweet_count]], bump = twitter_user.bump)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
    let twitter_user:  &mut Account<TwitterUser> = &mut ctx.accounts.twitter_user;
    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
    let clock: Clock = Clock::get().unwrap();
    if topic.as_bytes().len() > 64 {
        return Err(ErrorCode::TopicTooLong.into())
    }
    if content.as_bytes().len() > 512 {
        return Err(ErrorCode::ContentTooLong.into())
    }
    
    tweet.topic = topic;
    tweet.content = content;
    tweet.tweet_number = twitter_user.tweet_count;
    tweet.timestamp = clock.unix_timestamp;
    twitter_user.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}