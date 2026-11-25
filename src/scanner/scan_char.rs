use crate::error::CosmoErr::Scan;

use super::*;

impl Scanner {
    pub fn scan_char(&mut self) -> CosmoResult<()> {
        let c = self.advance();

        let Some(c) = c else {
            return Err(CosmoErr::Scan(ScanErr::NotFound {
                at: self.pointers.line,
                message: "Token was not found!".to_string(),
            }));
        };

        match c {
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Asterisk),
            '/' => self.add_token(TokenType::Slash),
            ' ' | '\r' | '\t' | '\n' => {
                if c == '\n' {
                    self.pointers.line += 1;
                }
            }
            _ => {
                return Err(Scan(ScanErr::Unrecognized {
                    at: self.pointers.line,
                    token: c.to_string(),
                }));
            }
        }

        Ok(())
    }
}
