use super::*;

#[derive(Debug)]
#[allow(warnings)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    line: u32,
}

pub type TokenContent = (String, TokenType, u32);

impl Token {
    pub fn new(content: TokenContent) -> Self {
        Self {
            lexeme: content.0,
            token_type: content.1,
            line: content.2,
        }
    }
}
