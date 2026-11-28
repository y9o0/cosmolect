use super::*;

pub trait Codgen {
    fn generate(self) -> CosmoResult<Gen>;
}

// a place houlder till i build a VM
pub enum Gen {
    Number(f64),
}

impl Gen {
    pub fn to_num(self) -> f64 {
        match self {
            Self::Number(n) => n,
        }
    }
}
