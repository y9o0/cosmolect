use super::*;

impl Codgen for LowExpr {
    fn generate(self) -> CosmoResult<Gen> {
        match self {
            Self::Number(n) => Ok(Gen::Number(n)),

            Self::Binary { lhs, op, rhs } => {
                let lhs = lhs.generate()?;
                let rhs = rhs.generate()?;

                Ok(match op {
                    TokenType::Plus => Gen::Number(lhs.to_num() + rhs.to_num()),
                    _ => return Err(CosmoErr::CodeGen(CodegenErr::BadToken { token: op })),
                })
            }
        }
    }
}
