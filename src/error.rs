use std::fmt;

use crate::scanner::TokenType;
pub type CosmoResult<T> = std::result::Result<T, CosmoErr>;

#[derive(Debug, thiserror::Error)]
pub enum CosmoErr {
    Scan(ScanErr),
    Parse(ParseErr),
    HirExpr(HirExprErr),
    CodeGen(CodegenErr),
}

impl fmt::Display for CosmoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scan(x) => write!(f, "{}", x),
            Self::Parse(x) => write!(f, "{}", x),
            Self::HirExpr(x) => write!(f, "{}", x),
            Self::CodeGen(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanErr {
    #[error("at : '{}',and due to '{}'", at, message)]
    NotFound { at: u32, message: String },
    #[error("at : '{}', and due to '{}' is unrecognized token!", at, token)]
    Unrecognized { at: u32, token: String },
    #[error(
        "at : '{}', and due to '{}' couldn't be parsed by the scanner!",
        at,
        txt
    )]
    Unconsumable { at: u32, txt: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ParseErr {
    #[error("Unexpected : at -> '{}' due to -> '{}'", at, message)]
    Unexpected { at: usize, message: String },
    #[error("BadToken : found at -> '{}' and was -> '{:?}'", at, token)]
    BadToken { at: usize, token: TokenType },
}

#[derive(Debug, thiserror::Error)]
pub enum HirExprErr {
    #[error("Unexpected : due to '{}'", message)]
    Unexpected { message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CodegenErr {
    #[error("BadToken : '{:?}'", token)]
    BadToken { token: TokenType },
}
