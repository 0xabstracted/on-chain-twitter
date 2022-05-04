use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
     // --------------------------------------- generic (0 - 19)
     #[msg("failed to perform some math operation safely")]
     ArithmeticError, //0x1770
 
     #[msg("unknown instruction called")]
     UnknownInstruction,
 
     #[msg("invalid parameter passed")]
     InvalidParameter,
 
     #[msg("anchor serialization issue")]
     AnchorSerializationIssue,
 
     #[msg("two amounts that are supposed to be equal are not")]
     AmountMismatch,
 
     #[msg("account discriminator doesn't match")]
     AccountDiscriminatorMismatch,
     Reserved6,
     Reserved7,
     Reserved8,
     Reserved9,
     Reserved10,
     Reserved11,
     Reserved12,
     Reserved13,
     Reserved14,
     Reserved15,
     Reserved16,
     Reserved17,
     Reserved18,
     Reserved19,
// String length errors 
    #[msg(" Topic length is greater than 64!!")]
    TopicTooLong,
    #[msg(" Content length is greater than 512!!")]
    ContentTooLong,
    #[msg(" Username length is greater than 64!!")]
    UsernameTooLong,
    #[msg("New Username length is greater than 64!!")]
    NewUsernameTooLong,
}