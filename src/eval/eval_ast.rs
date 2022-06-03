use super::parser::{BindStruct, LocalBindStruct, Node};
use crate::{Bounds, Ty};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(super) struct Output {
    // Representing bound names only for binding expressions.
    name: Option<String>,
    ty: Ty,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let Output { name, ty } = self;
        match name {
            Some(name) => write!(f, "val {} : ", name)?,
            None => write!(f, "- : ")?,
        }
        match ty {
            Ty::Int(int) => write!(f, "int = {}", int),
            Ty::Bool(b) => write!(f, "bool = {}", if *b { "true" } else { "false" }),
            Ty::List(list) => {
                write!(f, "int list = ")?;
                list.fmt(f)
            }
            Ty::Fn => write!(f, "... = <fun>"),
        }
    }
}

pub(super) fn eval_ast(ast: &Node, bounds: &mut Bounds) -> Result<Output, String> {
    match ast {
        Node::Int(i) => Ok(Output {
            name: None,
            ty: Ty::Int(*i),
        }),
        Node::Bool(b) => Ok(Output {
            name: None,
            ty: Ty::Bool(*b),
        }),
        Node::List(list) => Ok(Output {
            name: None,
            ty: Ty::List(list.clone()),
        }),
        Node::Add(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    ty: Ty::Int(l),
                },
                Output {
                    name: None,
                    ty: Ty::Int(r),
                },
            ) => Ok(Output {
                name: None,
                ty: Ty::Int(l + r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Sub(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    ty: Ty::Int(l),
                },
                Output {
                    name: None,
                    ty: Ty::Int(r),
                },
            ) => Ok(Output {
                name: None,
                ty: Ty::Int(l - r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Mul(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    ty: Ty::Int(l),
                },
                Output {
                    name: None,
                    ty: Ty::Int(r),
                },
            ) => Ok(Output {
                name: None,
                ty: Ty::Int(l * r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Div(lhs, rhs) => match (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?) {
            (
                Output {
                    name: None,
                    ty: Ty::Int(l),
                },
                Output {
                    name: None,
                    ty: Ty::Int(r),
                },
            ) => Ok(Output {
                name: None,
                ty: Ty::Int(l / r),
            }),
            _ => Err("This expression has a type other than int".to_string()),
        },
        Node::Eql(lhs, rhs) => {
            if let (
                Output {
                    name: None,
                    ty: lty,
                },
                Output {
                    name: None,
                    ty: rty,
                },
            ) = (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?)
            {
                match (lty, rty) {
                    (Ty::Int(l), Ty::Int(r)) => {
                        return Ok(Output {
                            name: None,
                            ty: Ty::Bool(l == r),
                        })
                    }
                    (Ty::Bool(l), Ty::Bool(r)) => {
                        return Ok(Output {
                            name: None,
                            ty: Ty::Bool(l == r),
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
                    ty: lty,
                },
                Output {
                    name: None,
                    ty: rty,
                },
            ) = (eval_ast(lhs, bounds)?, eval_ast(rhs, bounds)?)
            {
                match (lty, rty) {
                    (Ty::Int(l), Ty::Int(r)) => {
                        return Ok(Output {
                            name: None,
                            ty: Ty::Bool(l != r),
                        })
                    }
                    (Ty::Bool(l), Ty::Bool(r)) => {
                        return Ok(Output {
                            name: None,
                            ty: Ty::Bool(l != r),
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
                ty: value.clone(),
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
                    Output { name: None, ty } => ty,
                    _ => return Err("Syntax error".to_string()),
                }
            } else {
                Ty::Fn
            };
            bounds.bind(name.clone(), value.clone());
            Ok(Output {
                name: Some(name),
                ty: value,
            })
        }
        Node::LocalBind(local_bind) => {
            let LocalBindStruct { bind, scope } = &**local_bind;
            // Eval local binding.
            let BindStruct {
                name,
                args: _,
                expr,
            } = &*bind;
            let name = match name {
                Node::Ident(name) => name.clone(),
                _ => return Err("Expected a value name".to_string()),
            };
            // TODO: args
            let value = match eval_ast(expr, bounds)? {
                Output { name: None, ty } => ty,
                _ => return Err("Syntax error".to_string()),
            };
            // Make new bound values from global bound values.
            let mut bounds_locally = bounds.clone();
            bounds_locally.bind(name, value);
            // Eval expresion in scope with local bindings.
            match eval_ast(scope, &mut bounds_locally)? {
                Output {
                    name: None,
                    ty: Ty::Int(i),
                } => Ok(Output {
                    name: None,
                    ty: Ty::Int(i),
                }),
                _ => Err("Syntax error".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ty::List;
    use std::collections::HashMap;

    #[test]
    fn eval_int() {
        let ast = Node::Int(123);
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            ty: Ty::Int(123),
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
            ty: Ty::Int(16),
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
            ty: Ty::Int(123),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Ty::Int(123))])),
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
            ty: Ty::Int(987),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Ty::Int(987))])),
        );
    }

    #[test]
    fn eval_bound_global_value() {
        // foo
        let ast = Node::Ident("foo".to_string());
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Ty::Int(123))]));
        let expected = Output {
            name: None,
            ty: Ty::Int(123),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Ty::Int(123))]))
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
            ty: Ty::Int(7),
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
        let mut bounds = Bounds(HashMap::from([("foo".to_string(), Ty::Int(123))]));
        let expected = Output {
            name: None,
            ty: Ty::Int(7),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("foo".to_string(), Ty::Int(123))]))
        );
    }

    #[test]
    fn eval_empty_list() {
        // []
        let ast = Node::List(List::new());
        let mut bounds = Bounds::new();
        let expected = Output {
            name: None,
            ty: Ty::List(List::new()),
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
            ty: Ty::List(list),
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
            ty: Ty::List(list.clone()),
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            Bounds(HashMap::from([("lst".to_string(), Ty::List(list))]))
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
            ty: Ty::Bool(true),
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
            ty: Ty::Bool(false),
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
            ty: Ty::Bool(true),
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
            ty: Ty::Bool(false),
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
            ty: Ty::Bool(true),
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
            ty: Ty::Bool(true),
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
            ty: Ty::Bool(false),
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
            ty: Ty::Fn,
        };
        let actual = eval_ast(&ast, &mut bounds).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(
            bounds,
            // TODO
            Bounds(HashMap::from([("square".to_string(), Ty::Fn)]))
        );
    }
}
