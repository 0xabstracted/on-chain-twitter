use anchor_lang::prelude::*;
use crate::state::*;
use oct_common::error::ErrorCode;

#[derive(Accounts)]
pub struct SendTweet <'info>{
    #[account(
        mut,
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()],
        bump = twitter_user.bump
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    #[account(
        init, 
        payer = author, 
        space = Tweet::LEN,
        seeds = [b"tweet-account".as_ref(), author.key().as_ref(), twitter_user.next_address.key().as_ref()],
        bump
    )]
    //#[account(init, payer = author, space = 8 + Tweet::LEN,seeds = [b"tweet-account".as_ref(), author.key().as_ref(),&[twitter_user.tweet_count]], bump)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
    let twitter_user: &mut Account<TwitterUser> = &mut ctx.accounts.twitter_user;
    let clock: Clock = Clock::get().unwrap();
    if topic.as_bytes().len() > 64 {
        return Err(ErrorCode::TopicTooLong.into())
    }
    if content.as_bytes().len() > 512 {
        return Err(ErrorCode::ContentTooLong.into())
    }

    tweet.address = twitter_user.next_address ;
    tweet.timestamp = clock.unix_timestamp;
    tweet.topic = topic;
    tweet.content = content;
    tweet.author = *ctx.accounts.author.key;
    twitter_user.tweet_count +=1;
    tweet.tweet_number = twitter_user.tweet_count;
    tweet.bump = *ctx.bumps.get("tweet").unwrap();
    msg!("tweet.tweet_number.to_le_bytes().as_ref(): { }]",tweet.tweet_number);
    twitter_user.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}