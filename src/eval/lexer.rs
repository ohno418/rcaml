#[derive(Debug, PartialEq)]
pub(super) enum Token {
    Int(i64),      // integer
    Punct(String), // punctuator
}

pub(super) fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut rest = input;
    loop {
        let c = match rest.chars().next() {
            Some(c) => c,
            None => break,
        };

        // whitespaces
        if c.is_ascii_whitespace() {
            // skip white spaces
            rest = &rest[1..];
            continue;
        }

        // punctuators
        if c.is_ascii_punctuation() {
            match c {
                '+' | '-' | '*' => {
                    tokens.push(Token::Punct(c.to_string()));
                    rest = &rest[1..];
                    continue;
                }
                _ => (),
            }
        }

        // integer
        if c.is_ascii_digit() {
            let int;
            (int, rest) = take_integer_from(rest)?;
            tokens.push(Token::Int(int));
            continue;
        }

        return Err(format!(r#"Failed to tokenize "{}""#, rest));
    }
    Ok(tokens)
}

fn take_integer_from(s: &str) -> Result<(i64, &str), String> {
    let mut rest = s;
    let mut int_str = "".to_string();
    loop {
        match rest.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                int_str.push(c);
                rest = &rest[1..];
            }
            _ => break,
        }
    }
    match int_str.parse() {
        Ok(int) => Ok((int, rest)),
        Err(_) => Err(format!(r#"Failed to parse "{}" into i64"#, s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_integer() {
        let input = "123";
        let expected = vec![Token::Int(123)];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_arithmetic_expr() {
        let input = "2+3*4-5";
        let expected = vec![
            Token::Int(2),
            Token::Punct("+".to_string()),
            Token::Int(3),
            Token::Punct("*".to_string()),
            Token::Int(4),
            Token::Punct("-".to_string()),
            Token::Int(5),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_tokenize_not_integer() {
        let input = "123abc";
        assert!(tokenize(input).is_err());
    }
}
