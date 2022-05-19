mod eval;
mod read;

use eval::eval;
use read::{read, ReadError};

// global variable
#[derive(Debug, PartialEq)]
struct GVar {
    name: String,
    value: i64,
}

pub fn repl() -> Result<(), String> {
    // TODO: use hashmap
    let mut gvars: Vec<GVar> = Vec::new();

    loop {
        let input = match read() {
            Ok(input) => input,
            Err(ReadError::CtrlD) => {
                println!("");
                break;
            }
            Err(ReadError::Unknown) => return Err("failed to read an input".to_string()),
        };

        match eval(input, &mut gvars) {
            Ok(output) => println!("{}", output),
            Err(err) => println!("Error: {}", err),
        }

        println!("global variables are: {:?}", gvars);
    }

    Ok(())
}
