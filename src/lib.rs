mod eval;
mod read;

use eval::eval;
use read::{read, ReadError};
use std::collections::HashMap;

// global bound values
#[derive(Debug, PartialEq)]
struct Vals(HashMap<String, i64>);

impl Vals {
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
    let mut vals = Vals::new();

    loop {
        let input = match read() {
            Ok(input) => input,
            Err(ReadError::CtrlD) => {
                println!("");
                break;
            }
            Err(ReadError::Unknown) => return Err("failed to read an input".to_string()),
        };

        match eval(input, &mut vals) {
            Ok(output) => println!("{}", output),
            Err(err) => println!("Error: {}", err),
        }

        println!("global values are: {:?}", vals);
    }

    Ok(())
}
