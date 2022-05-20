mod eval_ast;
mod lexer;
mod parser;

use super::Bounds;
use eval_ast::eval_ast;
use lexer::tokenize;
use parser::parse;

// <program> ::= <expr> ";;"
pub(crate) fn eval(input: String, bounds: &mut Bounds) -> Result<String, String> {
    // Extract an expression that precedes ";;".
    let expr = match input.find(";;") {
        Some(idx) => (&input[0..idx]).trim(),
        None => return Err(r#"";;" is required at the end of a expression"#.to_string()),
    };

    let tokens = tokenize(expr)?;
    let ast = parse(&tokens)?;
    let output = eval_ast(&ast, bounds)?;
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn eval_integer_input() {
        let input = "123;;".to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_input_with_spaces() {
        let input = " 123  ;;".to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_arithmetic_expr() {
        let input = "2+3*4+5-6/2;;".to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 16";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_global_binding() {
        let input = "let foo = 42;;".to_string();
        let mut bounds = Bounds::new();
        let expected = "val foo : int = 42";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 42)])),);
    }

    #[test]
    fn overwrites_existing_global_binding() {
        let input = "let foo = 123;;".to_string();
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), 42)]));
        let expected = "val foo : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 123)])),);
    }

    #[test]
    fn eval_existing_global_binding() {
        let input = "foo ;;".to_string();
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), 456)]));
        let expected = "- : int = 456";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 456)])),);
    }

    #[test]
    fn eval_local_binding() {
        let input = "let lv = 345 in lv + 2 ;;".to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 347";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_nested_local_bindings_1() {
        let input = r#"
            let a = 1 in
            let b = 2 in
            a ;;
        "#
        .to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 1";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_nested_local_bindings_2() {
        let input = r#"
            let a = 1 in
            let b = 2 in
            b ;;
        "#
        .to_string();
        let mut bounds = Bounds::new();
        let expected = "- : int = 2";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }
}
