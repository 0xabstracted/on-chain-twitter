use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct DeleteTwitterAccount<'info>{
    #[account(
        mut, 
        seeds = [b"twitter-user".as_ref(),author.key().as_ref()], 
        bump = twitter_user.bump, 
        close = author
    )]
    pub twitter_user: Account<'info, TwitterUser>,
    #[account(mut)]
    pub author: Signer<'info>,
}

pub fn handler(_ctx: Context<DeleteTwitterAccount>) -> Result<()> {
    Ok(())
}