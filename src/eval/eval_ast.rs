use super::parser::Node;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Output {
    Int(i64), // int
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Int(int) => write!(f, "- : int = {}", int),
        }
    }
}

pub(super) fn eval_ast(ast: &Node) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output::Int(*i)),
        Node::Add(lhs, rhs) => {
            let (Output::Int(l), Output::Int(r)) = (eval_ast(lhs)?, eval_ast(rhs)?);
            Ok(Output::Int(l + r))
        }
        Node::Sub(lhs, rhs) => {
            let (Output::Int(l), Output::Int(r)) = (eval_ast(lhs)?, eval_ast(rhs)?);
            Ok(Output::Int(l - r))
        }
        Node::Mul(lhs, rhs) => {
            let (Output::Int(l), Output::Int(r)) = (eval_ast(lhs)?, eval_ast(rhs)?);
            Ok(Output::Int(l * r))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let expected = Output::Int(123);
        let actual = eval_ast(&ast).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_arithmetic_expr() {
        // 2+3*4-5
        let ast = Node::Sub(
            Box::new(Node::Add(
                Box::new(Node::Int(2)),
                Box::new(Node::Mul(
                    Box::new(Node::Int(3)),
                    Box::new(Node::Int(4)),
                )),
            )),
            Box::new(Node::Int(5)),
        );
        let expected = Output::Int(9);
        let actual = eval_ast(&ast).unwrap();
        assert_eq!(expected, actual);
    }
}
