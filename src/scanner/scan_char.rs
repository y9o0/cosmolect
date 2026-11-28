use crate::error::CosmoErr::Scan;

use super::*;

impl Scanner {
    pub fn scan_char(&mut self) -> CosmoResult<()> {
        let c = self.advance()?;

        match c {
            '+' => {
                if self.match_next('-') {
                    self.add_token(TokenType::PlusMinus);
                } else {
                    self.add_token(TokenType::Plus)
                }
            }
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Asterisk),
            '/' => self.add_token(TokenType::Slash),
            '±' => self.add_token(TokenType::PlusMinus),
            '#' => {
                while self.peek(0)? != '\n' && !self.is_at_end() {
                    self.advance()?;
                }
            }
            x if x.is_numeric() => self.numbers()?,
            x if x.is_alphabetic() => self.keyword()?,
            '\n' => {
                self.pointers.line += 1;
                self.add_token(TokenType::NewLine)
            }
            ' ' | '\r' | '\t' => {}
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
