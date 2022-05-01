use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct TransferOwnershipUserAccount<'info>{
    #[account(mut, has_one= author, seeds = [b"twitter_user".as_ref(), author.key().as_ref()], bump = twitter_user_account.bump)]
    pub twitter_user_account: Account<'info, TwitterUser>,
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<TransferOwnershipUserAccount>, new_owner: Pubkey) -> Result<()>{
    let twitter_user_account = &mut ctx.accounts.twitter_user_account;
        let clock: Clock = Clock::get().unwrap();
        //if new_owner.as_bytes().len() > 64 {
         //   return Err(ErrorCode::NewUsernameTooLong.into())    
        //  
          
        twitter_user_account.author = new_owner;
        twitter_user_account.last_interaction_timestamp = clock.unix_timestamp;
        Ok(())
    
}
