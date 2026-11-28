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

pub fn compile(source: &str) -> CosmoResult<()> {
    let scanner = Scanner::new(source.to_string()).scan()?;
    let mut parser = Parser::new(scanner);

    let mut hirexprs = Vec::new();
    while parser.peek()?.token_type != TokenType::Eof {
        let parsed = parser.parse(0)?;

        if parser.peek()?.token_type == TokenType::NewLine && parsed != HirExpr::NewLine(()) {
            hirexprs.push(parsed);
        }
    }

    let mut lowered_exprs: Vec<LowExpr> = Vec::new();

    for e in hirexprs {
        let e = e.to_lower_expr()?;
        lowered_exprs.push(e);
    }

    let mut generated = Vec::new();
    for le in lowered_exprs {
        let gen_e = le.generate()?;
        generated.push(gen_e);
    }

    for i in generated {
        let Gen::Number(n) = i;
        println!("{}", n)
    }

    Ok(())
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

        self.pointers.previous = std::mem::take(&mut self.pointers.current);
        self.pointers.current = std::mem::take(index);

        self.pointers.line += 1;

        Ok(())
    }

    pub fn peek(&self) -> CosmoResult<&Token> {
        let index = self.pointers.line;

        let Some(tok) = self.tokens.get(index) else {
            return Err(Parse(ParseErr::Unexpected {
                at: self.pointers.line,
                message: format!("Couldn't peek at {}", index),
            }));
        };

        Ok(tok)
    }
}
