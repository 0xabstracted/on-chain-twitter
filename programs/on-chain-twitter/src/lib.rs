use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;


use instructions::*;

declare_id!("5C9Wsr23aNXpRjumomx1aMLtEp1bYEjSw7LEUGDYAVPv");


#[program]
pub mod on_chain_twitter {
    use super::*;
    pub fn create_twitter_account(ctx: Context<CreateTwitterAccount>, username: String) -> Result<()> {
        instructions::create_twitter_account::handler(ctx,username)
    }
    pub fn change_user_name(ctx: Context<ChangeUserName>, username: String) -> Result<()>{
        instructions::change_user_name::handler(ctx,username)
    }
    pub fn transfer_ownership_user_account(ctx: Context<TransferOwnershipUserAccount>, new_owner: Pubkey) -> Result<()>{
        instructions::transfer_ownership_user_account::handler(ctx, new_owner)
    }
    //pub fn get_number_of_tweets_by_user(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u64>{
    //pub fn get_number_of_tweets_by_user(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u32>{
    pub fn get_number_of_tweets_by_user(ctx: Context<GetNumberOfTweetsByUser>) -> Result<u8>{
        instructions::get_number_of_tweets_by_user::handler(ctx)
    }
    pub fn delete_twitter_account(_ctx: Context<DeleteTwitterAccount>) -> Result<()> {
        instructions::delete_twitter_account::handler(_ctx)
    }
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        instructions::send_tweet::handler(ctx, topic, content)
    }
    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
        instructions::update_tweet::handler(ctx, topic, content)
    }
    pub fn delete_tweet(ctx: Context<DeleteTweet>) -> Result<()> {
        instructions::delete_tweet::handler(ctx)
    }
    pub fn update_next_address(ctx: Context<UpdateNextAddress>, next_address: Pubkey) -> Result<()>{
        instructions::update_next_address::handler(ctx, next_address)
    }
}





