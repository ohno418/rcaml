use super::{
    parser::{BindStruct, LocalBindStruct, Node},
    Value,
};
use crate::Bounds;
use std::fmt;

#[derive(Debug, PartialEq)]
pub(super) struct Output {
    // Representing bound names only for binding expressions.
    name: Option<String>,
    value: Value,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let Output { name, value } = self;
        match name {
            Some(name) => write!(f, "val {} : ", name)?,
            None => write!(f, "- : ")?,
        }
        match value {
            Value::Int(int) => write!(f, "int = {}", int),
            Value::Bool(b) => write!(f, "bool = {}", if *b { "true" } else { "false" }),
            Value::List(list) => {
                write!(f, "int list = ")?;
                list.fmt(f)
            }
            Value::Fn => write!(f, "... = <fun>"),
        }
    }
}

pub(super) fn eval_ast(ast: &Node, bounds: &mut Bounds) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output {
            name: None,
            value: Value::Int(*i),
        }),
        Node::Bool(b) => Ok(Output {
            name: None,
            value: Value::Bool(*b),
        }),
        Node::List(list) => Ok(Output {
            name: None,
            value: Value::List(list.clone()),
        }),
        Node::Add(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    value: Value::Int(l),
                },
                Output {
                    name: None,
                    value: Value::Int(r),
                },
            ) => Ok(Output {
                name: None,
                value: Value::Int(l + r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Sub(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    value: Value::Int(l),
                },
                Output {
                    name: None,
                    value: Value::Int(r),
                },
            ) => Ok(Output {
                name: None,
                value: Value::Int(l - r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Mul(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    value: Value::Int(l),
                },
                Output {
                    name: None,
                    value: Value::Int(r),
                },
            ) => Ok(Output {
                name: None,
                value: Value::Int(l * r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Div(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    value: Value::Int(l),
                },
                Output {
                    name: None,
                    value: Value::Int(r),
                },
            ) => Ok(Output {
                name: None,
                value: Value::Int(l / r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Eql(lhs, rhs) => {
            if let (
                Output {
                    name: None,
                    value: lval,
                },
                Output {
                    name: None,
                    value: rval,
                },
            ) = (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?)
            {
                match (lval, rval) {
                    (Value::Int(l), Value::Int(r)) => {
                        return Ok(Output {
                            name: None,
                            value: Value::Bool(l == r),
                        })
                    }
                    (Value::Bool(l), Value::Bool(r)) => {
                        return Ok(Output {
                            name: None,
                            value: Value::Bool(l == r),
                        })
                    }
                    _ => (),
                }
            }
            Err("Syntax error".to_string())
        }
        Node::Neql(lhs, rhs) => {
            if let (
                Output {
                    name: None,
                    value: lval,
                },
                Output {
                    name: None,
                    value: rval,
                },
            ) = (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?)
            {
                match (lval, rval) {
                    (Value::Int(l), Value::Int(r)) => {
                        return Ok(Output {
                            name: None,
                            value: Value::Bool(l != r),
                        })
                    }
                    (Value::Bool(l), Value::Bool(r)) => {
                        return Ok(Output {
                            name: None,
                            value: Value::Bool(l != r),
                        })
                    }
                    _ => (),
                }
            }
            Err("Syntax error".to_string())
        }
        Node::Ident(name) => match bounds.get(name) {
            Some(value) => Ok(Output {
                name: None,
                value: value.clone(),
            }),
            None => Err(format!("Unbound value {}", name)),
        },
        Node::Bind(bind) => {
            let BindStruct { name, args, expr } = &**bind;
            let name = match name {
                Node::Ident(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            // TODO: args
            let value = if args.is_empty() {
                match eval_ast(expr, bounds)? {
                    Output { name: None, value } => value,
                    _ => return Err("Syntax error".to_string()),
                }
            } else {
                Value::Fn
            };
            bounds.bind(name.clone(), value.clone());
            Ok(Output {
                name: Some(name),
                value,
            })
        }
        Node::LocalBind(local_bind) => {
            let LocalBindStruct { bind, scope } = &**local_bind;
            // Eval local binding.
            let BindStruct { name, args, expr } = &*bind;
            let name = match name {
                Node::Ident(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            let value = if args.is_empty() {
                match eval_ast(expr, bounds)? {
                    Output { name: None, value } => value,
                    _ => return Err("Syntax error".to_string()),
                }
            } else {
                Value::Fn
            };
            // Make new bound values from global bound values.
            let mut bounds_locally = bounds.clone();
            bounds_locally.bind(name, value);
            // Eval expresion in scope with local bindings.
            match eval_ast(scope, &mut bounds_locally)? {
                Output {
                    name: None,
                    value: Value::Int(i),
                } => Ok(Output {
                    name: None,
                    value: Value::Int(i),
                }),
                _ => Err("Syntax error".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::value::List;
    use std::collections::HashMap;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Int(123),
        };
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
        let expected = Output {
            name: None,
            value: Value::Int(16),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_global_binding() {
        // let foo = 123
        let ast = Node::Bind(Box::new(BindStruct {
            name: Node::Ident("foo".to_string()),
            args: vec![],
            expr: Node::Int(123),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: Some("foo".to_string()),
            value: Value::Int(123),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(123))])),
        );
    }

    #[test]
    fn overwrites_existing_global_binding() {
        // let foo = 123
        let ast = Node::Bind(Box::new(BindStruct {
            name: Node::Ident("foo".to_string()),
            args: vec![],
            expr: Node::Int(987),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: Some("foo".to_string()),
            value: Value::Int(987),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(987))])),
        );
    }

    #[test]
    fn eval_bound_global_value() {
        // foo
        let ast = Node::Ident("foo".to_string());
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Value::Int(123))]));
        let expected = Output {
            name: None,
            value: Value::Int(123),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(123))]))
        );
    }

    #[test]
    fn eval_local_binding() {
        // let x = 5 in x + 2
        let ast = Node::LocalBind(Box::new(LocalBindStruct {
            bind: BindStruct {
                name: Node::Ident("x".to_string()),
                args: vec![],
                expr: Node::Int(5),
            },
            scope: Node::Add(
                Box::new(Node::Ident("x".to_string())),
                Box::new(Node::Int(2)),
            ),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Int(7),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn shadow_global_binding_with_local_binding() {
        // let foo = 5 in foo + 2
        let ast = Node::LocalBind(Box::new(LocalBindStruct {
            bind: BindStruct {
                name: Node::Ident("foo".to_string()),
                args: vec![],
                expr: Node::Int(5),
            },
            scope: Node::Add(
                Box::new(Node::Ident("foo".to_string())),
                Box::new(Node::Int(2)),
            ),
        }));
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Value::Int(123))]));
        let expected = Output {
            name: None,
            value: Value::Int(7),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Value::Int(123))]))
        );
    }

    #[test]
    fn eval_empty_list() {
        // []
        let ast = Node::List(List::new());
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::List(List::new()),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_list() {
        // [1; 2; 3]
        let list = List(
            Some(1),
            Some(Box::new(List(
                Some(2),
                Some(Box::new(List(Some(3), Some(Box::new(List(None, None)))))),
            ))),
        );
        let ast = Node::List(list.clone());
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::List(list),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }

    #[test]
    fn eval_list_binding() {
        // let lst = [1; 2; 3]
        let list = List(
            Some(1),
            Some(Box::new(List(
                Some(2),
                Some(Box::new(List(Some(3), Some(Box::new(List(None, None)))))),
            ))),
        );
        let ast = Node::Bind(Box::new(BindStruct {
            name: Node::Ident("lst".to_string()),
            args: vec![],
            expr: Node::List(list.clone()),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: Some("lst".to_string()),
            value: Value::List(list.clone()),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("lst".to_string(), Value::List(list))]))
        );
    }

    #[test]
    fn error_on_arithmetic_operation_for_not_int_pair() {
        // [1; 2] + 3
        let ast = Node::Add(
            Box::new(Node::List(List::from(&vec![1, 2]))),
            Box::new(Node::Int(3)),
        );
        assert!(eval_ast(&ast, &mut Bounds::new()).is_err());
    }

    #[test]
    fn eval_true() {
        // true
        let ast = Node::Bool(true);
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(true),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_equal_1() {
        // 2 == 3
        let ast = Node::Eql(Box::new(Node::Int(2)), Box::new(Node::Int(3)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(false),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_equal_2() {
        // 3 == 3
        let ast = Node::Eql(Box::new(Node::Int(3)), Box::new(Node::Int(3)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(true),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_equal_3() {
        // true == false
        let ast = Node::Eql(Box::new(Node::Bool(true)), Box::new(Node::Bool(false)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(false),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_equal_4() {
        // false == false
        let ast = Node::Eql(Box::new(Node::Bool(false)), Box::new(Node::Bool(false)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(true),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_not_equal_1() {
        // 2 != 3
        let ast = Node::Neql(Box::new(Node::Int(2)), Box::new(Node::Int(3)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(true),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_not_equal_2() {
        // 3 != 3
        let ast = Node::Neql(Box::new(Node::Int(3)), Box::new(Node::Int(3)));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Bool(false),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eval_func_definition() {
        // let square x = x * x
        let ast = Node::Bind(Box::new(BindStruct {
            name: Node::Ident("square".to_string()),
            args: vec![Node::Ident("x".to_string())],
            expr: Node::Mul(
                Box::new(Node::Ident("x".to_string())),
                Box::new(Node::Ident("x".to_string())),
            ),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: Some("square".to_string()),
            value: Value::Fn,
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            // TODO
            Bounds(HashMap::from([("square".to_string(), Value::Fn)]))
        );
    }

    #[test]
    fn eval_local_func_definition() {
        // let square x = x * x in 42
        let ast = Node::LocalBind(Box::new(LocalBindStruct {
            bind: BindStruct {
                name: Node::Ident("square".to_string()),
                args: vec![Node::Ident("x".to_string())],
                expr: Node::Mul(
                    Box::new(Node::Ident("x".to_string())),
                    Box::new(Node::Ident("x".to_string())),
                ),
            },
            scope: Node::Int(42),
        }));
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            value: Value::Int(42),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(bounds, Bounds::new());
    }
}
