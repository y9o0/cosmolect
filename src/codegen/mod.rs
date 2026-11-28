pub(in crate::codegen) mod core;
pub(in crate::codegen) mod generate;

use crate::error::{CodegenErr, CosmoErr, CosmoResult};
use crate::scanner::TokenType;
use crate::sema::LowExpr;
pub use core::{Codgen, Gen};
