use std::{
    ffi::OsStr,
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::Path,
};

use scanner::core::Scanner;

mod error;
mod scanner;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut code = String::new();

    if let Some(x) = path {
        let x = Path::new(&x);

        if x.extension().unwrap_or(OsStr::new(&"Unknown extension")) != "csl" {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "Unknown extension; make sure it's ax before trying again",
            ));
        }

        code = read_to_string(x)?;
    }

    let s = Scanner::new(code).scan().unwrap();

    for i in s {
        println!("{:?}", i);
    }

    Ok(())
}
