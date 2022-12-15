pub use axum;
pub use serde::*;
pub use serde_derive::{self, *};
pub use shared_string::{self, *};
pub use thiserror::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
