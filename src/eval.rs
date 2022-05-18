mod lexer;
mod parser;
mod eval_ast;

use lexer::tokenize;
use parser::parse;
use eval_ast::eval_ast;

// <program> ::= <expr> ";;"
pub fn eval(input: String) -> Result<String, String> {
    let expr = match input.find(";;") {
        Some(idx) => (&input[0..idx]).trim(),
        None => return Err(r#"";;" is required at the end of a expression"#.to_string()),
    };

    let tokens = tokenize(expr)?;
    let ast = parse(&tokens)?;
    let output = eval_ast(&ast)?;
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer_input() {
        let input = "123;;".to_string();
        let expected = "- : int = 123";
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
        let expected = "- : int = 123";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_addition() {
        let input = "2+3;;".to_string();
        let expected = "- : int = 5";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_subtract() {
        let input = "6-2;;".to_string();
        let expected = "- : int = 4";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }
}
