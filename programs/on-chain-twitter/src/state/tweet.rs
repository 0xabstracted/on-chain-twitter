use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct Tweet{
    pub address: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub author: Pubkey,
//    pub tweet_number: u64,
//    pub tweet_number: u32,
    pub tweet_number: u8,
    pub bump: u8,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 64;
const MAX_CONTENT_LENGTH: usize = 512;
const MAX_PUBKEY_LENGTH: usize = 32;
//const MAX_TWEET_NUMBER_SIZE: usize = 8;
//const MAX_TWEET_NUMBER_SIZE: usize = 4;
const MAX_TWEET_NUMBER_SIZE: usize = 1;
const BUMP_LENGTH: usize = 1;
impl Tweet{
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
                + MAX_PUBKEY_LENGTH
                + MAX_TIMESTAMP_SIZE 
                + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
                + STRING_LENGTH_PREFIX +  MAX_CONTENT_LENGTH 
                + MAX_PUBKEY_LENGTH
                + MAX_TWEET_NUMBER_SIZE
                + BUMP_LENGTH;
}
