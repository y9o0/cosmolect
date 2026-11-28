mod core;
mod exprs;

use crate::error::{CosmoErr, CosmoResult, ParseErr};
use crate::scanner::{Token, core::Scanner};
pub use core::{Parser, compile};

use crate::{
    codegen::{Codgen, Gen},
    error::CosmoErr::Parse,
    scanner::TokenType,
    sema::{HirExpr, LowExpr},
};
