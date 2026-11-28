use super::*;

impl Parser {
    pub fn numbers(&mut self) -> CosmoResult<HirExpr> {
        let num = &self.pointers.current.lexeme;

        let Ok(num) = num.parse::<f64>() else {
            return Err(CosmoErr::Parse(ParseErr::Unexpected {
                at: self.pointers.line,
                message: format!("Couldn't parse the lexeme into a number -> '{}' ", num),
            }));
        };

        Ok(HirExpr::Number(num))
    }
}
