use crate::{error::CosmoErr::Parse, sema::HirExpr};

use super::*;

impl Parser {
    pub fn parse(&mut self, min_bp: u8) -> CosmoResult<HirExpr> {
        self.advance()?;

        let lhs = match &self.pointers.current.token_type {
            TokenType::Numbers => self.numbers()?,
            TokenType::NewLine => return Ok(HirExpr::NewLine(())),
            _ => {
                return Err(CosmoErr::Parse(ParseErr::Unexpected {
                    at: self.pointers.line,
                    message: format!("bad token '{:?}'", self.pointers.current.token_type),
                }));
            }
        };

        loop {
            let op = match self.peek()?.token_type {
                TokenType::Plus => TokenType::Plus,
                TokenType::Eof | TokenType::NewLine => break,
                other => {
                    return Err(CosmoErr::Parse(ParseErr::BadToken {
                        at: self.pointers.line,
                        token: other,
                    }));
                }
            };

            let (l_pb, r_bp) = self.infix_binding_power(op)?;

            if l_pb < min_bp {
                break;
            }

            self.advance()?;
            let rhs = self.parse(r_bp)?;

            return Ok(match op {
                TokenType::Plus => HirExpr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                },
                TokenType::Eof | TokenType::NewLine => break,
                other => {
                    return Err(CosmoErr::Parse(ParseErr::BadToken {
                        at: self.pointers.line,
                        token: other,
                    }));
                }
            });
        }

        Ok(lhs)
    }

    fn infix_binding_power(&self, op: TokenType) -> CosmoResult<(u8, u8)> {
        Ok(match op {
            TokenType::Plus | TokenType::Minus => (1, 2),
            TokenType::Asterisk | TokenType::Slash => (3, 4),
            _ => {
                return Err(Parse(ParseErr::BadToken {
                    at: self.pointers.line,
                    token: op,
                }));
            }
        })
    }
}
