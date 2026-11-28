use super::*;

pub enum LowExpr {
    Number(f64),

    Binary {
        lhs: Box<LowExpr>,
        op: TokenType,
        rhs: Box<LowExpr>,
    },
}
