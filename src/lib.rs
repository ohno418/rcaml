mod eval;
mod read;

use eval::{eval, Ty};
use read::{read, ReadError};
use std::collections::HashMap;

// bound values
#[derive(Clone, Debug, PartialEq)]
struct Bounds(HashMap<String, Ty>);

impl Bounds {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&self, name: &str) -> Option<&Ty> {
        let map = &self.0;
        map.get(name)
    }

    fn bind(&mut self, name: String, value: Ty) {
        let map = &mut self.0;
        map.insert(name, value);
    }
}

pub fn repl() -> Result<(), String> {
    let mut bounds = Bounds::new();

    loop {
        let input = match read() {
            Ok(input) => input,
            Err(ReadError::CtrlD) => {
                println!();
                break;
            }
            Err(ReadError::Unknown) => return Err("failed to read an input".to_string()),
        };

        match eval(input, &mut bounds) {
            Ok(output) => println!("{}", output),
            Err(err) => println!("Error: {}", err),
        }
    }

    Ok(())
}
