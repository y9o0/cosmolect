use crate::error::CosmoResult;
use scanner::core::Scanner;

mod error;
mod scanner;

fn main() -> CosmoResult<()> {
    let scan = Scanner::new("+ - / *".to_string()).scan()?;

    for i in scan {
        println!(">>{:?}", i);
    }

    Ok(())
}
