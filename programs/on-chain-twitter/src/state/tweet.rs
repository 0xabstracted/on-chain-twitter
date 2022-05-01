use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct Tweet{
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub author: Pubkey,
    pub tweet_number: u8,
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 64 *4 ;
const MAX_CONTENT_LENGTH: usize = 512 * 4;
const MAX_AUTHOR_LENGTH: usize = 32;
const MAX_TWEETS_PER_USER: usize = 1;
impl Tweet{
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
                + MAX_TIMESTAMP_SIZE 
                + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
                + STRING_LENGTH_PREFIX +  MAX_CONTENT_LENGTH 
                + MAX_AUTHOR_LENGTH
                + MAX_TWEETS_PER_USER;
}