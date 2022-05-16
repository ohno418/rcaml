pub fn eval(input: String) -> Result<String, String> {
    match input.parse::<i64>() {
        Ok(n) => Ok(n.to_string()),
        Err(_) => Err(format!(r#"Cannot parse "{}" into int"#, input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer_input() {
        let input = "123".to_string();
        let expected = "123";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_negative_integer_input() {
        let input = "-42".to_string();
        let expected = "-42";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_not_integer_input() {
        let input = "123abc".to_string();
        assert!(eval(input).is_err())
    }
}
