mod eval_ast;
mod lexer;
mod parser;
mod value;

use crate::Bounds;
use eval_ast::eval_ast;
use lexer::tokenize;
use parser::parse;
pub(crate) use value::Value;

pub(crate) fn eval(input: &str, bounds: &mut Bounds) -> Result<String, String> {
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
        let input = "123;;";
        let mut bounds = Bounds::new();
        let expected = "- : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_input_with_spaces() {
        let input = " 123  ;;";
        let mut bounds = Bounds::new();
        let expected = "- : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_arithmetic_expr() {
        let input = "2+3*4+5-6/2+(3-1)*2;;";
        let mut bounds = Bounds::new();
        let expected = "- : int = 20";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_global_binding() {
        let input = "let foo = 42;;";
        let mut bounds = Bounds::new();
        let expected = "val foo : int = 42";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(42))]))
        );
    }

    #[test]
    fn overwrites_existing_global_binding() {
        let input = "let foo = 123;;";
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Value::Int(42))]));
        let expected = "val foo : int = 123";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(123))]))
        );
    }

    #[test]
    fn eval_existing_global_binding() {
        let input = "foo;;";
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Value::Int(456))]));
        let expected = "- : int = 456";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(456))]))
        );
    }

    #[test]
    fn eval_local_binding() {
        let input = "let lv = 345 in lv + 2;;";
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
            a;;
        "#;
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
            b;;
        "#;
        let mut bounds = Bounds::new();
        let expected = "- : int = 2";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_empty_list() {
        let input = "[];;";
        let mut bounds = Bounds::new();
        let expected = "- : int list = []";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_list() {
        let input = "[1; 2; 3];;";
        let mut bounds = Bounds::new();
        let expected = "- : int list = [1; 2; 3]";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_true() {
        let input = "true;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = true";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_false() {
        let input = "false;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = false";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_equal_between_int_1() {
        let input = "2 == 3;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = false";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_equal_between_int_2() {
        let input = "3 == 3;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = true";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_not_equal_between_int_1() {
        let input = "2 != 3;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = true";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_not_equal_between_int_2() {
        let input = "3 != 3;;";
        let mut bounds = Bounds::new();
        let expected = "- : bool = false";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_func_definition() {
        let input = "let square x = x * x;;";
        let mut bounds = Bounds::new();
        // TODO
        let expected = "val square : ... = <fun>";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("square".to_string(), Value::Fn)])),
        );
    }

    #[test]
    fn eval_local_func_definition() {
        let input = "let add a b = a + b in 42;;";
        let mut bounds = Bounds::new();
        let expected = "- : int = 42";
        let actual = eval(input, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }
}
