use anchor_lang::prelude::*;

declare_id!("7cNGAP6tHh8W1BMxBEuMVfBUs8ogFT5PwUsENJLVhhjs");

#[program]
pub mod on_chain_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet_account;
        let clock: Clock = Clock::get().unwrap();
        msg!("&*ctx.accounts.author.key.to_string() {}",&*ctx.accounts.author.key.to_string());
        msg!("B tweet.timestamp: {}",tweet.timestamp);
        msg!("B tweet.topic: {}",tweet.topic);
        msg!("B tweet.content: {}",tweet.content);
        msg!("B tweet.bump: {}",tweet.bump);
        if topic.as_bytes().len() > 69 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 420 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        tweet.timestamp = clock.unix_timestamp;
        msg!("tweet.timestamp: {}",tweet.timestamp);
        tweet.topic = topic;
        msg!("tweet.topic: {}",tweet.topic);
        msg!("topic.as_bytes().len(): {}",tweet.topic.as_bytes().len());
        tweet.content = content;
        msg!("tweet.content: {}",tweet.content);
        msg!("content.as_bytes().len(): {}",tweet.content.as_bytes().len());
        tweet.bump = *ctx.bumps.get("tweet_account").unwrap();
        msg!("tweet.bump: {}",tweet.bump);
        Ok(())
    }
    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet_account;

        if topic.as_bytes().len() > 69 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 420 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        tweet.topic = topic;
        tweet.content = content;
        Ok(())
    }
    pub fn delete_tweet(_ctx: Context<DeleteTweet>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct SendTweet <'info>{
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = Tweet::LEN, seeds = [b"tweet_author", author.key().as_ref()], bump)]
    //#[account(init, payer = author, space = Tweet::LEN, seeds = [b"tweet_author"], bump)]
    pub tweet_account: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTweet <'info>{
    #[account(mut, seeds = [b"tweet_author", author.key().as_ref()], bump = tweet_account.bump)]
    //#[account(mut, has_one = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    #[account(mut, seeds = [b"tweet_author", author.key().as_ref()], bump = tweet_account.bump, close = author)]
   // #[account(mut, has_one = author, close = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[account]
pub struct Tweet{
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub bump: u8,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 69;
const MAX_CONTENT_LENGTH: usize = 420;
const SEED_LENGTH: usize = 1;
impl Tweet{
    const LEN: usize = DISCRIMINATOR_LENGTH + 
                MAX_TIMESTAMP_SIZE + 
                STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH + 
                STRING_LENGTH_PREFIX +  MAX_CONTENT_LENGTH + 
                SEED_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg(" Topic length is greater than 69!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 420!!")]
    ContentTooLong,
}