use anchor_lang::prelude::*;

declare_id!("5BAvFQDCEafYDwEdRut8VmFpQEA5gb78kSH55G82vMzB");

#[program]
pub mod on_chain_twitter {
    use super::*;
    pub fn create_twitter_account(ctx: Context<CreateTwitterAccount>, username: String) -> Result<()> {
        let twitter_user_account = &mut ctx.accounts.twitter_user_account;
        if username.as_bytes().len() > 64 {
            return Err(ErrorCode::UsernameTooLong.into())
        }
        twitter_user_account.tweet_count = 0;
        twitter_user_account.authority = *ctx.accounts.authority.key;
        twitter_user_account.username = username;    
        twitter_user_account.bump = *ctx.bumps.get("twitter_user_account").unwrap();
     
        Ok(())
    }
    pub fn change_user_name(ctx: Context<ChangeUserName>, username: String) -> Result<()>{
        let twitter_user_account = &mut ctx.accounts.twitter_user_account;
        if username.as_bytes().len() > 64 {
            return Err(ErrorCode::NewUsernameTooLong.into())    
        }
        twitter_user_account.username = username;
        Ok(())
    }
    pub fn delete_twitter_account(_ctx: Context<DeleteTwitterAccount>) -> Result<()> {
        Ok(())
    }
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet_account;
        let twitter_user_account = &mut ctx.accounts.twitter_user_account;
        let clock: Clock = Clock::get().unwrap();
        if topic.as_bytes().len() > 64 {
            return Err(ErrorCode::TopicTooLong.into())
        }
        if content.as_bytes().len() > 512 {
            return Err(ErrorCode::ContentTooLong.into())
        }
        twitter_user_account.tweet_count +=1;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
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
pub struct CreateTwitterAccount <'info>{
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = TwitterUser::LEN, seeds = [b"twitter_user".as_ref(), authority.key().as_ref()], bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ChangeUserName <'info>{
    #[account(mut, has_one = authority, seeds = [b"twitter_user".as_ref(),authority.key().as_ref()], bump = twitter_user_account.bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct DeleteTwitterAccount<'info>{
    #[account(mut, has_one = authority, seeds = [b"twitter_user".as_ref(), authority.key().as_ref()], bump = twitter_user_account.bump, close = authority)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SendTweet <'info>{
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet_account: Account<'info, Tweet>,
    #[account(mut,seeds = [b"twitter_user".as_ref(), author.key().as_ref()], bump = twitter_user_account.bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTweet <'info>{
    #[account(mut,has_one = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    #[account(mut, has_one = author, close = author)]
    pub tweet_account: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[account]
pub struct Tweet{
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub author: Pubkey,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 64;
const MAX_CONTENT_LENGTH: usize = 512;
const MAX_AUTHOR_LENGTH: usize = 32;
impl Tweet{
    const LEN: usize = DISCRIMINATOR_LENGTH 
                + MAX_TIMESTAMP_SIZE 
                + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
                + STRING_LENGTH_PREFIX +  MAX_CONTENT_LENGTH 
                + MAX_AUTHOR_LENGTH;
}

#[account]
pub struct TwitterUser{
    pub tweet_count: u64,
    pub authority: Pubkey,
    pub username: String,    
    pub bump: u8,
}
const MAX_TWEET_COUNT_SIZE: usize = 8;
const MAX_AUTHORITY_LENGTH: usize = 32;
const MAX_USERNAME_LENGTH: usize = 64;
const BUMP_LENGTH: usize = 1;
impl TwitterUser{
    const LEN: usize =  DISCRIMINATOR_LENGTH 
                + MAX_TWEET_COUNT_SIZE
                + MAX_AUTHORITY_LENGTH
                + STRING_LENGTH_PREFIX + MAX_USERNAME_LENGTH
                +  BUMP_LENGTH;
}
#[error_code]
pub enum ErrorCode {
    #[msg(" Topic length is greater than 64!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 512!!")]
    ContentTooLong,
    #[msg(" Username length is greater than 64!!")]
    UsernameTooLong,
    #[msg("New Username length is greater than 64!!")]
    NewUsernameTooLong,
}