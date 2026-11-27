use super::*;

pub struct Parser {
    tokens: Vec<Token>,
    pub pointers: Pointers,
}

pub struct Pointers {
    pub current: Token,
    pub previous: Token,
    pub line: usize,
}

impl Pointers {
    pub fn new() -> Self {
        Self {
            current: Token::none(),
            previous: Token::none(),
            line: 0,
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pointers: Pointers::new(),
        }
    }

    pub fn advance(&mut self) -> CosmoResult<()> {
        let Some(index) = self.tokens.get_mut(self.pointers.line as usize) else {
            return Err(CosmoErr::Parse(ParseErr::Unexpected {
                at: self.pointers.line,
                message: "Couldn't advance the pointer".to_string(),
            }));
        };

        self.pointers.current = std::mem::take(index);
        self.pointers.previous = std::mem::take(&mut self.pointers.current);

        self.pointers.line += 1;

        Ok(())
    }
}
