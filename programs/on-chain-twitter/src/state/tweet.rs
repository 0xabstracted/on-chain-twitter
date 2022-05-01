use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct Tweet{
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub author: Pubkey,
    pub tweet_number: u64,
}
