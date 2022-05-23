use std::io;
use std::io::Write;

pub(super) enum ReadError {
    CtrlD,
    Unknown,
}

pub(super) fn read() -> Result<String, ReadError> {
    print!("# ");
    io::stdout().flush().expect("failed to flush");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                Err(ReadError::CtrlD)
            } else {
                Ok(input.trim().to_string())
            }
        }
        Err(_) => Err(ReadError::Unknown),
    }
}
