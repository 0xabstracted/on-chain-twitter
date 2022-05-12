use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct TwitterUser{
    //pub tweet_count: u64,
    //pub tweet_count: u32,
    pub author: Pubkey,
    pub last_interaction_timestamp: i64, 
    pub next_address: Pubkey,   
    pub username: String,
    pub bump: u8,
    pub tweet_count: u8,
    _reserved: [u8; 10],
}
//const MAX_TWEET_COUNT_SIZE: usize = 8;
//const MAX_TWEET_COUNT_SIZE: usize = 4;
const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_TIMESTAMP_SIZE: usize = 8; 
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_USERNAME_LENGTH: usize = 64 ;
const MAX_TWEET_COUNT_SIZE: usize = 1;
const BUMP_LENGTH: usize = 1;
const RESERVED_SIZE: usize = 10;

impl TwitterUser{
    pub const LEN: usize = MAX_TWEET_COUNT_SIZE
                + MAX_PUBKEY_LENGTH
                + STRING_LENGTH_PREFIX + MAX_USERNAME_LENGTH
                + MAX_TIMESTAMP_SIZE
                + MAX_PUBKEY_LENGTH
                + BUMP_LENGTH
                + RESERVED_SIZE;
}

