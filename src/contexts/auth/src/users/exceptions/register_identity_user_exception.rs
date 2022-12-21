use common::*;

// #[derive(Debug)]
// pub struct RegisterIdentityUserException(String);

#[derive(Error, Debug)]
pub enum AuthContextError {
    #[error("RegisterIdentityUserException: {0}")]
    RegisterIdentityUserException(String),
    #[error("unknown data store error")]
    Unknown,
}
