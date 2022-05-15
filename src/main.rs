use rcaml::repl;
use std::process;

fn main() {
    match repl() {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
