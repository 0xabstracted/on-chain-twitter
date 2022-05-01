use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg(" Topic length is greater than 64!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 512!!")]
    ContentTooLong,
    #[msg(" Username length is greater than 64!!")]
    UsernameTooLong,
    #[msg("New Username length is greater than 64!!")]
    NewUsernameTooLong,
}