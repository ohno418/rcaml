#[derive(Debug, PartialEq)]
pub(super) enum Token {
    Int(i64),      // integer
    Punct(String), // punctuator
    Kw(KwKind),    // keyword
    Ident(String), // identifier
}

#[derive(Debug, PartialEq)]
pub(super) enum KwKind {
    Let,  // let
    In,   // in
    True, // true
}

pub(super) fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut rest = input;
    while let Some(c) = rest.chars().next() {
        // whitespaces
        if c.is_ascii_whitespace() {
            // skip white spaces
            rest = &rest[1..];
            continue;
        }

        // integers
        if c.is_ascii_digit() {
            let int;
            (int, rest) = take_integer_from(rest)?;
            tokens.push(Token::Int(int));
            continue;
        }

        // punctuators
        if c.is_ascii_punctuation() {
            match c {
                '+' | '-' | '*' | '/' | '=' | '[' | ']' | ';' => {
                    tokens.push(Token::Punct(c.to_string()));
                    rest = &rest[1..];
                    continue;
                }
                _ => (),
            }
        }

        // identifiers and keywords
        if let Ok((ident, r)) = take_identifier_from(rest) {
            let tok = match &*ident {
                "let" => Token::Kw(KwKind::Let),
                "in" => Token::Kw(KwKind::In),
                "true" => Token::Kw(KwKind::True),
                _ => Token::Ident(ident),
            };
            tokens.push(tok);
            rest = r;
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

fn take_identifier_from(s: &str) -> Result<(String, &str), String> {
    let mut rest = s;
    let mut ident: Option<String> = None;
    loop {
        match rest.chars().next() {
            Some(c) if c.is_ascii_alphabetic() => {
                ident = match ident {
                    Some(mut ident) => {
                        ident.push(c);
                        Some(ident)
                    }
                    None => Some(c.to_string()),
                };
                rest = &rest[1..];
            }
            _ => break,
        }
    }
    match ident {
        Some(ident) => Ok((ident, rest)),
        None => Err("Failed to take an identifier".to_string()),
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
        let input = "2+3*4+5-6/2";
        let expected = vec![
            Token::Int(2),
            Token::Punct("+".to_string()),
            Token::Int(3),
            Token::Punct("*".to_string()),
            Token::Int(4),
            Token::Punct("+".to_string()),
            Token::Int(5),
            Token::Punct("-".to_string()),
            Token::Int(6),
            Token::Punct("/".to_string()),
            Token::Int(2),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_global_binding() {
        let input = "let foo = 123";
        let expected = vec![
            Token::Kw(KwKind::Let),
            Token::Ident("foo".to_string()),
            Token::Punct("=".to_string()),
            Token::Int(123),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_local_binding() {
        let input = "let x = 5 in x";
        let expected = vec![
            Token::Kw(KwKind::Let),
            Token::Ident("x".to_string()),
            Token::Punct("=".to_string()),
            Token::Int(5),
            Token::Kw(KwKind::In),
            Token::Ident("x".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_empty_list() {
        let input = "[]";
        let expected = vec![Token::Punct("[".to_string()), Token::Punct("]".to_string())];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_list() {
        let input = "[1; 2; 3]";
        let expected = vec![
            Token::Punct("[".to_string()),
            Token::Int(1),
            Token::Punct(";".to_string()),
            Token::Int(2),
            Token::Punct(";".to_string()),
            Token::Int(3),
            Token::Punct("]".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_true() {
        let input = "true";
        let expected = vec![Token::Kw(KwKind::True)];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }
}
