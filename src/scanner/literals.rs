use super::*;

impl Scanner {
    pub(in crate::scanner) fn numbers(&mut self) -> CosmoResult<()> {
        // consume the whole stream of numbers
        while self.peek(0)?.is_numeric() {
            self.advance()?;
        }

        if self.peek(0)? == '.' && self.peek(1)?.is_numeric() {
            // just consume the .
            self.advance()?;

            // what and if what after the dot is numbers we keep consuming
            while self.peek(0)?.is_numeric() {
                self.advance()?;
            }
        }

        self.add_token(TokenType::Numbers);

        Ok(())
    }

    pub(in crate::scanner) fn keyword(&mut self) -> CosmoResult<()> {
        // while we are at an aplhabetic we keep consuming till the end of the stream
        while self.peek(0)?.is_alphabetic() {
            self.advance()?;
        }

        // we take that stream
        let txt = self
            .source
            .get(self.pointers.start as usize..self.pointers.current as usize);

        // then we extract it
        let Some(txt) = txt else {
            return Err(CosmoErr::Scan(ScanErr::Unconsumable {
                at: self.pointers.line,
                txt: txt.unwrap().iter().collect::<String>(),
            }));
        };

        // collect it back into string
        let str_txt = txt.iter().collect::<String>();

        // then we match on it, and if there is no match we return Idntifier token.
        match str_txt {
            _ => self.add_token(TokenType::Identifier),
        }

        Ok(())
    }
}
