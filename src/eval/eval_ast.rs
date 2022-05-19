use super::parser::Node;
use super::Vals;
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

pub(super) fn eval_ast(ast: &Node, vals: &mut Vals) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output::Int(*i)),
        Node::Add(lhs, rhs) => match (eval_ast(lhs, vals)?, eval_ast(rhs, vals)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l + r)),
            _ => unreachable!(),
        },
        Node::Sub(lhs, rhs) => match (eval_ast(lhs, vals)?, eval_ast(rhs, vals)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l - r)),
            _ => unreachable!(),
        },
        Node::Mul(lhs, rhs) => match (eval_ast(lhs, vals)?, eval_ast(rhs, vals)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l * r)),
            _ => unreachable!(),
        },
        Node::Div(lhs, rhs) => match (eval_ast(lhs, vals)?, eval_ast(rhs, vals)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l / r)),
            _ => unreachable!(),
        },
        Node::Bind(lhs, rhs) => {
            let name = match &**lhs {
                Node::Val(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            let value = match eval_ast(rhs, vals)? {
                Output::Int(i) => i,
                _ => unreachable!(),
            };
            vals.bind(name.clone(), value);
            Ok(Output::Val(name, value))
        }
        Node::Val(name) => match vals.get(name) {
            Some(value) => Ok(Output::Int(value)),
            None => Err(format!("Unbound value {}", name)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let mut vals = Vals::new();
        let expected = Output::Int(123);
        let actual = eval_ast(&ast, &mut vals).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_arithmetic_expr() {
        // 2+3*4+5-6/2
        let ast = Node::Sub(
            Box::new(Node::Add(
                Box::new(Node::Add(
                    Box::new(Node::Int(2)),
                    Box::new(Node::Mul(Box::new(Node::Int(3)), Box::new(Node::Int(4)))),
                )),
                Box::new(Node::Int(5)),
            )),
            Box::new(Node::Div(Box::new(Node::Int(6)), Box::new(Node::Int(2)))),
        );
        let mut vals = Vals::new();
        let expected = Output::Int(16);
        let actual = eval_ast(&ast, &mut vals).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_global_binding() {
        // let foo = 123
        let ast = Node::Bind(
            Box::new(Node::Val("foo".to_string())),
            Box::new(Node::Int(123)),
        );
        let mut vals = Vals::new();
        let expected = Output::Val("foo".to_string(), 123);
        let actual = eval_ast(&ast, &mut vals).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(vals, Vals(HashMap::from([("foo".to_string(), 123)])),);
    }

    #[test]
    fn overwrites_existing_global_binding() {
        // let foo = 123
        let ast = Node::Bind(
            Box::new(Node::Val("foo".to_string())),
            Box::new(Node::Int(987)),
        );
        let mut vals = Vals::new();
        let expected = Output::Val("foo".to_string(), 987);
        let actual = eval_ast(&ast, &mut vals).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(vals, Vals(HashMap::from([("foo".to_string(), 987)])),);
    }

    #[test]
    fn eval_bound_global_value() {
        // foo
        let ast = Node::Val("foo".to_string());
        let mut vals = Vals(HashMap::from([("foo".to_string(), 123)]));
        let expected = Output::Int(123);
        let actual = eval_ast(&ast, &mut vals).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(vals, Vals(HashMap::from([("foo".to_string(), 123)])));
    }
}
