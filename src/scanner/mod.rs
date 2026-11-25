pub mod core;
mod scan_char;
pub mod tokens;

use crate::error::{CosmoErr, CosmoResult, ScanErr};

use core::Scanner;

use tokens::{
    core::{Token, TokenContent},
    r#type::TokenType,
};
