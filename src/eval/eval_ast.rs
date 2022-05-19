use super::parser::Node;
use super::GVar;
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Output {
    Int(i64),         // int
    Val(String, i64), // val
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Int(int) => write!(f, "- : int = {}", int),
            Self::Val(name, value) => write!(f, "val {} : int = {}", name, value),
        }
    }
}

pub(super) fn eval_ast(ast: &Node, gvars: &mut Vec<GVar>) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output::Int(*i)),
        Node::Add(lhs, rhs) => match (eval_ast(lhs, gvars)?, eval_ast(rhs, gvars)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l + r)),
            _ => todo!(),
        },
        Node::Sub(lhs, rhs) => match (eval_ast(lhs, gvars)?, eval_ast(rhs, gvars)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l - r)),
            _ => todo!(),
        },
        Node::Mul(lhs, rhs) => match (eval_ast(lhs, gvars)?, eval_ast(rhs, gvars)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l * r)),
            _ => todo!(),
        },
        Node::Bind(lhs, rhs) => {
            let name = match &**lhs {
                Node::Var(name) => name.clone(),
                _ => return Err("Expected a variable name".to_string()),
            };
            let value = match eval_ast(rhs, gvars)? {
                Output::Int(i) => i,
                _ => todo!(),
            };
            let gvar = GVar {
                name: name.clone(),
                value,
            };
            // TODO: overwrite if exists
            gvars.push(gvar);
            Ok(Output::Val(name, value))
        }
        Node::Var(_name) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let mut gvars: Vec<GVar> = Vec::new();
        let expected = Output::Int(123);
        let actual = eval_ast(&ast, &mut gvars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_arithmetic_expr() {
        // 2+3*4-5
        let ast = Node::Sub(
            Box::new(Node::Add(
                Box::new(Node::Int(2)),
                Box::new(Node::Mul(Box::new(Node::Int(3)), Box::new(Node::Int(4)))),
            )),
            Box::new(Node::Int(5)),
        );
        let mut gvars: Vec<GVar> = Vec::new();
        let expected = Output::Int(9);
        let actual = eval_ast(&ast, &mut gvars).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_variable_binding() {
        // let foo = 123
        let ast = Node::Bind(
            Box::new(Node::Var("foo".to_string())),
            Box::new(Node::Int(123)),
        );
        let mut gvars: Vec<GVar> = Vec::new();
        let expected = Output::Val("foo".to_string(), 123);
        let actual = eval_ast(&ast, &mut gvars).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            gvars,
            vec![GVar {
                name: "foo".to_string(),
                value: 123
            }]
        );
    }

    // TODO: overwrite global variable test
}
