use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct TwitterUser{
    pub tweet_count: u8,
    pub author: Pubkey,
    pub username: String,
    pub last_interaction_timestamp: i64,    
    pub bump: u8,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TWEET_COUNT_SIZE: usize = 1;
const MAX_AUTHORITY_LENGTH: usize = 32;
const MAX_USERNAME_LENGTH: usize = 64 * 4;
const BUMP_LENGTH: usize = 1;
impl TwitterUser{
    pub const LEN: usize =  DISCRIMINATOR_LENGTH 
                + MAX_TWEET_COUNT_SIZE
                + MAX_AUTHORITY_LENGTH
                + STRING_LENGTH_PREFIX + MAX_USERNAME_LENGTH
                + MAX_TIMESTAMP_SIZE
                + BUMP_LENGTH;
}

