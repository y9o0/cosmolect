pub mod core;
pub(in crate::scanner) mod literals;
mod scan_char;
pub(in crate::scanner) mod tokens;

use crate::error::{CosmoErr, CosmoResult, ScanErr};

use core::Scanner;

pub use tokens::{Token, TokenContent, TokenType};
