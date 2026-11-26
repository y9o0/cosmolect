use std::char;

use crate::scanner::tokens::r#type::TokenType;

use super::*;

pub struct Scanner {
    pub(in crate::scanner) source: Vec<char>,
    pub(in crate::scanner) tokens: Vec<Token>,
    pub(in crate::scanner) pointers: Pointers,
}

pub(in crate::scanner) struct Pointers {
    pub(in crate::scanner) start: u32,
    pub(in crate::scanner) current: u32,
    pub(in crate::scanner) line: u32,
}

impl Pointers {
    pub fn new() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            pointers: Pointers::new(),
        }
    }

    pub fn scan(&mut self) -> CosmoResult<Vec<Token>> {
        while !self.is_at_end() {
            self.pointers.start = self.pointers.current;
            self.scan_tokens()?;
        }

        // for end of the file to be empty
        self.pointers.start = self.pointers.current;

        self.add_token(TokenType::Eof);
        return Ok(std::mem::take(&mut self.tokens));
    }

    pub(in crate::scanner) fn is_at_end(&self) -> bool {
        self.pointers.current >= self.source.len() as u32
    }

    fn scan_tokens(&mut self) -> CosmoResult<()> {
        self.scan_char()?;

        Ok(())
    }

    pub(in crate::scanner) fn add_token(&mut self, token_type: TokenType) {
        let txt = &self.source[self.pointers.start as usize..self.pointers.current as usize];

        let content = TokenContent::from((
            txt.iter().collect::<String>(),
            token_type,
            self.pointers.line,
        ));

        self.tokens.push(Token::new(content));
    }

    pub(in crate::scanner) fn advance(&mut self) -> CosmoResult<char> {
        let tok = self.source.get(self.pointers.current as usize);
        self.pointers.current += 1;

        let Some(tok) = tok else {
            return Err(CosmoErr::Scan(ScanErr::NotFound {
                at: self.pointers.line,
                message: "Token was not found!".to_string(),
            }));
        };

        Ok(*tok)
    }

    pub(in crate::scanner) fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || &self.source[self.pointers.current as usize] != &expected {
            return false;
        }

        self.pointers.current += 1;
        true
    }

    pub(in crate::scanner) fn peek(&self, n: u32) -> CosmoResult<char> {
        let index = self.pointers.current + n;
        let c = self.source.get(index as usize);

        if n > 0 {
            if self.pointers.current + n >= self.source.len() as u32 {
                return Ok('\0');
            }
        }

        let Some(c) = c else {
            return Err(CosmoErr::Scan(ScanErr::NotFound {
                at: self.pointers.line,
                message: "Token was not found!".to_string(),
            }));
        };

        Ok(*c)
    }
}
