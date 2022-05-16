#[derive(Debug, PartialEq)]
pub(super) enum Token {
    Int(i64), // integer
}

pub(super) fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    match input.parse::<i64>() {
        Ok(n) => Ok(vec![Token::Int(n)]),
        Err(_) => Err(format!(r#"Cannot tokenize "{}" into int"#, input)),
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
    fn cannot_tokenize_not_integer() {
        let input = "123abc";
        assert!(tokenize(input).is_err());
    }
}
