use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct TransferOwnershipUserAccount<'info>{
    //#[account(mut, has_one= author, seeds = [b"twitter-user".as_ref(), author.key().as_ref()], bump = twitter_user.bump)]
    #[account(
        mut, 
        seeds = [b"twitter-user".as_ref(), author.key().as_ref()], 
        bump = twitter_user.bump
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<TransferOwnershipUserAccount>, new_owner: Pubkey) -> Result<()>{
    let twitter_user = &mut ctx.accounts.twitter_user;
        let clock: Clock = Clock::get().unwrap();
        //if new_owner.as_bytes().len() > 64 {
         //   return Err(ErrorCode::NewUsernameTooLong.into())    
        //  
          
        twitter_user.author = new_owner;
        twitter_user.last_interaction_timestamp = clock.unix_timestamp;
        Ok(())
    
}
