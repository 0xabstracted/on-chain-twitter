use anchor_lang::prelude::*;

declare_id!("5BAvFQDCEafYDwEdRut8VmFpQEA5gb78kSH55G82vMzB");

#[program]
pub mod on_chain_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet_account;
        let clock: Clock = Clock::get().unwrap();
        if topic.as_bytes().len() > 64 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 512 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        tweet.bump = *ctx.bumps.get("tweet_account").unwrap();
        tweet.author = *ctx.accounts.author.key;
        Ok(())
    }
    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet_account;
        if topic.as_bytes().len() > 64 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 512 {
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
    #[account(init, payer = author, space = Tweet::LEN, seeds = [b"tweet_author".as_ref()], bump)]
    pub tweet_account: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTweet <'info>{
    #[account(mut,has_one = author, seeds = [b"tweet_author".as_ref()], bump = tweet_account.bump)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    #[account(mut, has_one = author, seeds = [b"tweet_author".as_ref()], bump = tweet_account.bump, close = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[account]
pub struct Tweet{
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub author: Pubkey,
    pub bump: u8,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 64;
const MAX_CONTENT_LENGTH: usize = 512;
const MAX_AUTHOR_LENGTH: usize = 32;
const SEED_LENGTH: usize = 1;
impl Tweet{
    const LEN: usize = DISCRIMINATOR_LENGTH + 
                MAX_TIMESTAMP_SIZE + 
                STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH + 
                STRING_LENGTH_PREFIX +  MAX_CONTENT_LENGTH + 
                MAX_AUTHOR_LENGTH +
                SEED_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg(" Topic length is greater than 64!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 512!!")]
    ContentTooLong,
}