pub(in crate::scanner) mod core;
pub(in crate::scanner) mod r#type;

use crate::scanner::tokens::r#type::TokenType;
pub use core::{Token, TokenContent};
