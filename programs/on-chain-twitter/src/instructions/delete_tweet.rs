use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct DeleteTweet <'info>{
    #[account(
        mut, 
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user.bump
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    #[account(
        mut, 
        seeds = [b"tweet-account".as_ref(), author.key().as_ref(), twitter_user.next_address.key().as_ref()], 
        bump= tweet.bump, 
        close = author
    )]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}


pub fn handler(ctx: Context<DeleteTweet>) -> Result<()>{
    let twitter_user:  &mut Account<TwitterUser> = &mut ctx.accounts.twitter_user;
    let clock: Clock = Clock::get().unwrap();
    twitter_user.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}
