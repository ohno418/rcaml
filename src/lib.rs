mod eval;
mod read;

use eval::eval;
use read::{read, ReadError};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct GVars(HashMap<String, i64>);

impl GVars {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&self, name: &str) -> Option<i64> {
        let map = &self.0;
        map.get(name).copied()
    }

    fn bind(&mut self, name: String, value: i64) {
        let map = &mut self.0;
        map.insert(name, value);
    }
}

pub fn repl() -> Result<(), String> {
    let mut gvars = GVars::new();

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
