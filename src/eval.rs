// <program> ::= <expr> ";;"
pub fn eval(input: String) -> Result<String, String> {
    let expr = match input.find(";;") {
        Some(idx) => (&input[0..idx]).trim(),
        None => return Err(r#"";;" is required at the end of a expression"#.to_string()),
    };

    match expr.parse::<i64>() {
        Ok(n) => Ok(n.to_string()),
        Err(_) => Err(format!(r#"Cannot parse "{}" into int"#, expr)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer_input() {
        let input = "123;;".to_string();
        let expected = "123";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_negative_integer_input() {
        let input = "-42;;".to_string();
        let expected = "-42";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_not_integer_input() {
        let input = "123abc;;".to_string();
        assert!(eval(input).is_err())
    }

    #[test]
    fn parses_input_with_spaces() {
        let input = " 123  ;;".to_string();
        let expected = "123";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }
}
