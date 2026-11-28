use super::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HirExpr {
    Number(f64),

    Binary {
        lhs: Box<HirExpr>,
        op: TokenType,
        rhs: Box<HirExpr>,
    },

    NewLine(()),
}

impl HirExpr {
    pub fn to_lower_expr(self) -> CosmoResult<LowExpr> {
        match self {
            Self::Binary { lhs, op, rhs } => {
                let lhs = lhs.to_lower_expr()?;
                let rhs = rhs.to_lower_expr()?;

                Ok(LowExpr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            }

            Self::Number(n) => Ok(LowExpr::Number(n)),

            _ => {
                return Err(HirExpr(HirExprErr::Unexpected {
                    message: format!("{:?} is unrecognized expr", self),
                }));
            }
        }
    }
}
