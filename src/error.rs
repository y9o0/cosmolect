use std::fmt;

pub type CosmoResult<T> = std::result::Result<T, CosmoErr>;

#[derive(Debug, thiserror::Error)]
pub enum CosmoErr {
    Scan(ScanErr),
    Parse(ParseErr),
}

impl fmt::Display for CosmoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scan(x) => write!(f, "{}", x),
            Self::Parse(x) => write!(f, "{}", x),
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
}
