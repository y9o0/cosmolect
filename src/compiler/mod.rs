mod core;
mod exprs;

use crate::error::{CosmoErr, CosmoResult, ParseErr};
use crate::scanner::Token;
pub use core::Parser;
