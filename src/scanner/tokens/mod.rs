pub(in crate::scanner) mod core;
pub(in crate::scanner) mod r#type;

pub use {
    core::{Token, TokenContent},
    r#type::TokenType,
};
