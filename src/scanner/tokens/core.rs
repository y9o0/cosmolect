use super::*;

#[derive(Debug)]
#[allow(warnings)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: u32,
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

    pub fn none() -> Self {
        Self {
            lexeme: String::new(),
            token_type: TokenType::Nai,
            line: 0,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::none()
    }
}
