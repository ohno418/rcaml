mod eval;
mod read;

use eval::eval;
use read::{read, ReadError};

pub fn repl() -> Result<(), String> {
    loop {
        let input = match read() {
            Ok(input) => input,
            Err(ReadError::CtrlD) => {
                println!("");
                break;
            }
            Err(ReadError::Unknown) => return Err("failed to read an input".to_string()),
        };

        match eval(input) {
            Ok(output) => println!("{}", output),
            Err(err) => println!("Error: {}", err),
        }
    }

    Ok(())
}
