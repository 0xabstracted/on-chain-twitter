use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct TwitterUser{
    pub tweet_count: u64,
    pub author: Pubkey,
    pub username: String,
    pub last_interaction_timestamp: i64,    
    pub bump: u8,
}

