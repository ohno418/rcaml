mod eval;
mod read;

use eval::eval;
use read::{read, ReadError};

pub fn repl() -> Result<(), &'static str> {
    loop {
        let input = match read() {
            Ok(input) => input,
            Err(ReadError::CtrlD) => {
                println!("");
                break;
            }
            Err(ReadError::Unknown) => return Err("failed to read an input"),
        };
        println!("{}", eval(input)?);
    }

    Ok(())
}
