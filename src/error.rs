use std::fmt;

pub type CosmoResult<T> = std::result::Result<T, CosmoErr>;

#[derive(Debug, thiserror::Error)]
pub enum CosmoErr {
    Scan(ScanErr),
}

impl fmt::Display for CosmoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scan(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanErr {
    #[error("at '{}' due to '{}'", at, message)]
    NotFound { at: u32, message: String },
    #[error("at '{}' due to '{}' is unrecognized token!", at, token)]
    Unrecognized { at: u32, token: String },
}
