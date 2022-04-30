use anchor_lang::prelude::*;

declare_id!("7cNGAP6tHh8W1BMxBEuMVfBUs8ogFT5PwUsENJLVhhjs");

#[program]
pub mod on_chain_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet_account;
        let clock: Clock = Clock::get().unwrap();
        msg!("&*ctx.accounts.author.key.to_string() {}",&*ctx.accounts.author.key.to_string());
        msg!("B1 tweet.timestamp: {}",tweet.timestamp);
        msg!("B1 tweet.topic: {}",tweet.topic);
        msg!("B1 tweet.content: {}",tweet.content);
        msg!("B1 tweet.bump: {}",tweet.bump);
        if topic.as_bytes().len() > 69 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 420 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        tweet.timestamp = clock.unix_timestamp;
        msg!("A1 tweet.timestamp: {}",tweet.timestamp);
        tweet.topic = topic;
        msg!("A1 tweet.topic: {}",tweet.topic);
        msg!("A1 topic.as_bytes().len(): {}",tweet.topic.as_bytes().len());
        tweet.content = content;
        msg!("A1 tweet.content: {}",tweet.content);
        msg!("A1 content.as_bytes().len(): {}",tweet.content.as_bytes().len());
        tweet.bump = *ctx.bumps.get("tweet_account").unwrap();
        msg!("A1 tweet.bump: {}",tweet.bump);
        tweet.author = *ctx.accounts.author.key;
        msg!("A1 tweet.author: {}",tweet.author);
        msg!("A1 tweet.author: {}",tweet.author.to_string());
        Ok(())
    }
    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet_account;
        msg!("B2 tweet.timestamp: {}",tweet.timestamp);
        msg!("B2 tweet.topic: {}",tweet.topic);
        msg!("B2 tweet.content: {}",tweet.content);
        msg!("B2 tweet.bump: {}",tweet.bump);
        msg!("B2 topic.as_bytes().len(): {}",tweet.topic.as_bytes().len());
        msg!("B2 content.as_bytes().len(): {}",tweet.content.as_bytes().len());
        
        if topic.as_bytes().len() > 69 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 420 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        tweet.topic = topic;
        tweet.content = content;
        msg!("A2 tweet.timestamp: {}",tweet.timestamp);
        msg!("A2 tweet.topic: {}",tweet.topic);
        msg!("A2 tweet.content: {}",tweet.content);
        msg!("A2 tweet.bump: {}",tweet.bump);
        msg!("A2 topic.as_bytes().len(): {}",tweet.topic.as_bytes().len());
        msg!("A2 content.as_bytes().len(): {}",tweet.content.as_bytes().len());
        Ok(())
    }
    pub fn delete_tweet(ctx: Context<DeleteTweet>) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet_account;
        msg!("D tweet.timestamp: {}",tweet.timestamp);
        msg!("D tweet.topic: {}",tweet.topic);
        msg!("D tweet.content: {}",tweet.content);
        msg!("D tweet.bump: {}",tweet.bump);
        msg!("D topic.as_bytes().len(): {}",tweet.topic.as_bytes().len());
        msg!("D content.as_bytes().len(): {}",tweet.content.as_bytes().len());
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
    #[account(mut,has_one = author, seeds = [b"tweet_author", author.key().as_ref()], bump = tweet_account.bump)]
    //#[account(mut, has_one = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    #[account(mut, has_one = author, seeds = [b"tweet_author", author.key().as_ref()], bump = tweet_account.bump, close = author)]
   // #[account(mut, has_one = author, close = author)]
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
const MAX_TOPIC_LENGTH: usize = 69;
const MAX_CONTENT_LENGTH: usize = 420;
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
    #[msg(" Topic length is greater than 69!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 420!!")]
    ContentTooLong,
}