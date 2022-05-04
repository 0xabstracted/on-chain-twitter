use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateNextAddress<'info>{
    pub author: Signer<'info>,
    #[account(
        mut, 
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user_account.bump
    )]
    pub twitter_user_account: Account<'info, TwitterUser>,
}


pub fn handler(ctx: Context<UpdateNextAddress>, next_address: Pubkey) -> Result<()>{
    let twitter_user_account = &mut ctx.accounts.twitter_user_account;
    let clock: Clock = Clock::get().unwrap();
    //if username.as_bytes().len() > 64 {
    //    return Err(ErrorCode::NewUsernameTooLong.into())    
    //}
    twitter_user_account.next_address = next_address;
    twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
    Ok(())
}