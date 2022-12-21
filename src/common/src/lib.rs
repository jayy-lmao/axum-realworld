pub use axum;
pub use serde::*;
pub use serde_derive::{self, *};
pub use shared_string::{self, *};
pub use sqlx;
pub use sqlx::{FromRow, Pool, Postgres};
pub use thiserror::Error;
pub use thiserror;
pub use async_trait;

mod errors {
    pub use thiserror::*;

#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("[Application error]")]
    DbError(String),
    #[error("[Bad user input]")]
    BadInput(String),
    #[error("[Not found error] {0}")]
    NotFound(String),
    #[error("[Parsing error]")]
    ParseError(String),
}

pub type AppResult<T> = Result<T, ApplicationError>;

}
pub use errors::*;

mod command_handler {
    use crate::AppResult;


#[async_trait::async_trait]
pub trait CommandHandler<T> {
    type Output;
    async fn command(&self, cmd: T) -> AppResult<Self::Output >;
}
}

pub use command_handler::CommandHandler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
