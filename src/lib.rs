use std::io;
use std::io::Write;

pub fn repl() -> Result<(), &'static str> {
    let input = {
        print!("> ");
        io::stdout().flush().expect("failed to flush");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().to_string(),
            Err(_) => return Err("failed to read an input"),
        }
    };

    println!("input = {}", input);

    Ok(())
}
