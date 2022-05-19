mod eval_ast;
mod lexer;
mod parser;

use super::GVar;
use eval_ast::eval_ast;
use lexer::tokenize;
use parser::parse;

// <program> ::= <expr> ";;"
pub(crate) fn eval(input: String, gvars: &mut Vec<GVar>) -> Result<String, String> {
    let expr = match input.find(";;") {
        Some(idx) => (&input[0..idx]).trim(),
        None => return Err(r#"";;" is required at the end of a expression"#.to_string()),
    };

    let tokens = tokenize(expr)?;
    let ast = parse(&tokens)?;
    let output = eval_ast(&ast, gvars)?;
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_integer_input() {
        let input = "123;;".to_string();
        let mut gvars: Vec<GVar> = vec![];
        let expected = "- : int = 123";
        let actual = eval(input, &mut gvars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_eval_not_integer_input() {
        let input = "123abc;;".to_string();
        let mut gvars: Vec<GVar> = vec![];
        assert!(eval(input, &mut gvars).is_err())
    }

    #[test]
    fn eval_input_with_spaces() {
        let input = " 123  ;;".to_string();
        let mut gvars: Vec<GVar> = vec![];
        let expected = "- : int = 123";
        let actual = eval(input, &mut gvars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_add_sub() {
        let input = "2+3+4-5;;".to_string();
        let mut vars: Vec<GVar> = vec![];
        let expected = "- : int = 4";
        let actual = eval(input, &mut vars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_expr_with_multiply() {
        let input = "2+3*4-5;;".to_string();
        let mut vars: Vec<GVar> = vec![];
        let expected = "- : int = 9";
        let actual = eval(input, &mut vars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_variable_binding() {
        let input = "let a = 42;;".to_string();
        let mut vars: Vec<GVar> = vec![];
        let expected = "val a : int = 42";
        let actual = eval(input, &mut vars).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            vars,
            vec![GVar {
                name: "a".to_string(),
                value: 42
            }]
        );
    }

    // TODO: overwrite global variable test
}
