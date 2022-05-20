use super::parser::{LocalBindStruct, Node};
use super::Bounds;
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

pub(super) fn eval_ast(ast: &Node, bounds: &mut Bounds) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output::Int(*i)),
        Node::Add(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l + r)),
            _ => unreachable!(),
        },
        Node::Sub(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l - r)),
            _ => unreachable!(),
        },
        Node::Mul(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l * r)),
            _ => unreachable!(),
        },
        Node::Div(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (Output::Int(l), Output::Int(r)) => Ok(Output::Int(l / r)),
            _ => unreachable!(),
        },
        Node::Val(name) => match bounds.get(name) {
            Some(value) => Ok(Output::Int(value)),
            None => Err(format!("Unbound value {}", name)),
        },
        Node::Bind(lhs, rhs) => {
            let name = match &**lhs {
                Node::Val(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            let value = match eval_ast(rhs, bounds)? {
                Output::Int(i) => i,
                _ => unreachable!(),
            };
            bounds.bind(name.clone(), value);
            Ok(Output::Val(name, value))
        }
        Node::LocalBind(local_bind) => {
            let LocalBindStruct { bind, scope } = &**local_bind;
            // Eval local binding.
            let (lhs, rhs) = bind;
            let name = match lhs {
                Node::Val(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            let value = match eval_ast(rhs, bounds)? {
                Output::Int(i) => i,
                _ => unreachable!(),
            };
            // Make new bound values from global bound values.
            let mut bounds_locally = bounds.clone();
            bounds_locally.bind(name, value);
            // Eval expresion in scope with local bindings.
            match eval_ast(scope, &mut bounds_locally)? {
                Output::Int(i) => Ok(Output::Int(i)),
                _ => Err("Syntax error".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let mut bounds = Bounds::new();
        let expected = Output::Int(123);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
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
        let mut bounds = Bounds::new();
        let expected = Output::Int(16);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_global_binding() {
        // let foo = 123
        let ast = Node::Bind(
            Box::new(Node::Val("foo".to_string())),
            Box::new(Node::Int(123)),
        );
        let mut bounds = Bounds::new();
        let expected = Output::Val("foo".to_string(), 123);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 123)])),);
    }

    #[test]
    fn overwrites_existing_global_binding() {
        // let foo = 123
        let ast = Node::Bind(
            Box::new(Node::Val("foo".to_string())),
            Box::new(Node::Int(987)),
        );
        let mut bounds = Bounds::new();
        let expected = Output::Val("foo".to_string(), 987);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 987)])),);
    }

    #[test]
    fn eval_bound_global_value() {
        // foo
        let ast = Node::Val("foo".to_string());
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), 123)]));
        let expected = Output::Int(123);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 123)])));
    }

    #[test]
    fn eval_local_binding() {
        // let x = 5 in x + 2
        let ast = Node::LocalBind(Box::new(LocalBindStruct {
            bind: (Node::Val("x".to_string()), Node::Int(5)),
            scope: Node::Add(Box::new(Node::Val("x".to_string())), Box::new(Node::Int(2))),
        }));
        let mut bounds = Bounds::new();
        let expected = Output::Int(7);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn shadow_global_binding_with_local_binding() {
        // let foo = 5 in foo + 2
        let ast = Node::LocalBind(Box::new(LocalBindStruct {
            bind: (Node::Val("foo".to_string()), Node::Int(5)),
            scope: Node::Add(
                Box::new(Node::Val("foo".to_string())),
                Box::new(Node::Int(2)),
            ),
        }));
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), 123)]));
        let expected = Output::Int(7);
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds(HashMap::from([("foo".to_string(), 123)])));
    }
}
