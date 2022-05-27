use super::lexer::{KwKind, Token};
use crate::ty::List;

#[derive(Debug, PartialEq)]
pub(super) enum Node {
    Int(i64),                        // integer
    Bool(bool),                      // boolean
    List(List),                      // list
    Add(Box<Node>, Box<Node>),       // +
    Sub(Box<Node>, Box<Node>),       // -
    Mul(Box<Node>, Box<Node>),       // *
    Div(Box<Node>, Box<Node>),       // /
    Eql(Box<Node>, Box<Node>),       // ==
    Val(String),                     // bound value
    Bind(Box<Node>, Box<Node>),      // global binding
    LocalBind(Box<LocalBindStruct>), // local binding
}

#[derive(Debug, PartialEq)]
pub(super) struct LocalBindStruct {
    pub bind: (Node, Node), // Bind node representing e.g. `let x = 42` part
    pub scope: Node,        // expression node in scope, followed by `in`
}

pub(super) fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (node, rest) = parse_expr(tokens)?;

    if !rest.is_empty() {
        return Err(format!("Found extra token: {:?}", rest));
    }

    Ok(node)
}

// <expr> ::= <bind>
fn parse_expr(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    parse_bind(tokens)
}

// <bind> ::= "let" identifier "=" <add> ("in" <expr>)?
//          | <add>
fn parse_bind(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    match tokens.get(0) {
        Some(Token::Kw(KwKind::Let)) => {
            let mut rest = &tokens[1..];
            let ident = match rest.get(0) {
                Some(Token::Ident(ident)) => {
                    rest = &rest[1..];
                    ident.clone()
                }
                _ => return Err("Expected an identifier".to_string()),
            };

            match rest.get(0) {
                Some(Token::Punct(p)) if p == "=" => rest = &rest[1..],
                _ => return Err("Expected =".to_string()),
            };

            let rhs;
            (rhs, rest) = parse_add(rest)?;

            match rest.get(0) {
                Some(Token::Kw(KwKind::In)) => {
                    let expr;
                    (expr, rest) = parse_expr(&rest[1..])?;
                    Ok((
                        Node::LocalBind(Box::new(LocalBindStruct {
                            bind: (Node::Val(ident), rhs),
                            scope: expr,
                        })),
                        rest,
                    ))
                }
                _ => Ok((Node::Bind(Box::new(Node::Val(ident)), Box::new(rhs)), rest)),
            }
        }
        _ => parse_add(tokens),
    }
}

// <add> ::= <mul> (("+" | "-") <mul>)*
fn parse_add(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_mul(tokens)?;

    while let Some(Token::Punct(p)) = rest.get(0) {
        match &**p {
            "+" => {
                let rhs;
                (rhs, rest) = parse_mul(&rest[1..])?;
                node = Node::Add(Box::new(node), Box::new(rhs));
            }
            "-" => {
                let rhs;
                (rhs, rest) = parse_mul(&rest[1..])?;
                node = Node::Sub(Box::new(node), Box::new(rhs));
            }
            _ => break,
        }
    }

    Ok((node, rest))
}

// <mul> ::= <equal> ("*" | "/" <equal>)*
fn parse_mul(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_equal(tokens)?;

    while let Some(Token::Punct(p)) = rest.get(0) {
        match &**p {
            "*" => {
                let rhs;
                (rhs, rest) = parse_equal(&rest[1..])?;
                node = Node::Mul(Box::new(node), Box::new(rhs));
            }
            "/" => {
                let rhs;
                (rhs, rest) = parse_equal(&rest[1..])?;
                node = Node::Div(Box::new(node), Box::new(rhs));
            }
            _ => break,
        }
    }

    Ok((node, rest))
}

// <equal> ::= <primary> ("==" <primary>)*
fn parse_equal(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_primary(tokens)?;

    while let Some(Token::Punct(p)) = rest.get(0) {
        if p != "==" {
            break;
        }
        let rhs;
        (rhs, rest) = parse_primary(&rest[1..])?;
        node = Node::Eql(Box::new(node), Box::new(rhs));
    }

    Ok((node, rest))
}

// <primary> ::= <int> | <boolean> | <val-name> | <list>
fn parse_primary(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    match tokens.get(0) {
        Some(Token::Int(int)) => Ok((Node::Int(*int), &tokens[1..])),
        Some(Token::Kw(KwKind::True)) => Ok((Node::Bool(true), &tokens[1..])),
        Some(Token::Kw(KwKind::False)) => Ok((Node::Bool(false), &tokens[1..])),
        Some(Token::Ident(name)) => Ok((Node::Val(name.clone()), &tokens[1..])),
        Some(Token::Punct(p)) if p == "[" => parse_list(tokens),
        _ => Err("Failed to parse a primary".to_string()),
    }
}

// <list> ::= "[" (<int> (";" <int>)*)? "]"
fn parse_list(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let mut rest = match tokens.get(0) {
        Some(Token::Punct(p)) if p == "[" => &tokens[1..],
        _ => return Err("Require [ to parse a list".to_string()),
    };
    let list = {
        let mut lst: Vec<i64> = Vec::new();
        let mut is_first = true;
        loop {
            match rest.get(0) {
                Some(Token::Punct(p)) if p == "]" => {
                    rest = &rest[1..];
                    break;
                }
                _ => (),
            }
            // skip ;
            if !is_first {
                match rest.get(0) {
                    Some(Token::Punct(p)) if p == ";" => rest = &rest[1..],
                    _ => return Err("; is required as a delimiter".to_string()),
                }
            }
            match rest.get(0) {
                Some(Token::Int(int)) => {
                    lst.push(*int);
                    rest = &rest[1..];
                    is_first = false;
                }
                _ => return Err("Failed to parse a list".to_string()),
            }
        }
        List::from(&lst)
    };
    Ok((Node::List(list), rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer() {
        let tokens = vec![Token::Int(42)];
        let expected = Node::Int(42);
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_arithmetic_expr() {
        // 2+3*4+5-6/2
        let tokens = vec![
            Token::Int(2),
            Token::Punct("+".to_string()),
            Token::Int(3),
            Token::Punct("*".to_string()),
            Token::Int(4),
            Token::Punct("+".to_string()),
            Token::Int(5),
            Token::Punct("-".to_string()),
            Token::Int(6),
            Token::Punct("/".to_string()),
            Token::Int(2),
        ];
        let expected = Node::Sub(
            Box::new(Node::Add(
                Box::new(Node::Add(
                    Box::new(Node::Int(2)),
                    Box::new(Node::Mul(Box::new(Node::Int(3)), Box::new(Node::Int(4)))),
                )),
                Box::new(Node::Int(5)),
            )),
            Box::new(Node::Div(Box::new(Node::Int(6)), Box::new(Node::Int(2)))),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_global_binding() {
        // let foo = 123
        let tokens = vec![
            Token::Kw(KwKind::Let),
            Token::Ident("foo".to_string()),
            Token::Punct("=".to_string()),
            Token::Int(123),
        ];
        let expected = Node::Bind(
            Box::new(Node::Val("foo".to_string())),
            Box::new(Node::Int(123)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_global_value_eval() {
        // foo
        let tokens = vec![Token::Ident("foo".to_string())];
        let expected = Node::Val("foo".to_string());
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_local_binding() {
        // let x = 5 in x + 2
        let tokens = vec![
            Token::Kw(KwKind::Let),
            Token::Ident("x".to_string()),
            Token::Punct("=".to_string()),
            Token::Int(5),
            Token::Kw(KwKind::In),
            Token::Ident("x".to_string()),
            Token::Punct("+".to_string()),
            Token::Int(2),
        ];
        let expected = Node::LocalBind(Box::new(LocalBindStruct {
            bind: (Node::Val("x".to_string()), Node::Int(5)),
            scope: Node::Add(Box::new(Node::Val("x".to_string())), Box::new(Node::Int(2))),
        }));
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_empty_list() {
        // []
        let tokens = vec![Token::Punct("[".to_string()), Token::Punct("]".to_string())];
        let expected = Node::List(List::new());
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_list() {
        // [1; 2; 3]
        let tokens = vec![
            Token::Punct("[".to_string()),
            Token::Int(1),
            Token::Punct(";".to_string()),
            Token::Int(2),
            Token::Punct(";".to_string()),
            Token::Int(3),
            Token::Punct("]".to_string()),
        ];
        let expected = Node::List(List(
            Some(1),
            Some(Box::new(List(
                Some(2),
                Some(Box::new(List(Some(3), Some(Box::new(List(None, None)))))),
            ))),
        ));
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_true() {
        let tokens = vec![Token::Kw(KwKind::True)];
        let expected = Node::Bool(true);
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_false() {
        let tokens = vec![Token::Kw(KwKind::False)];
        let expected = Node::Bool(false);
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_equal() {
        let tokens = vec![Token::Int(2), Token::Punct("==".to_string()), Token::Int(3)];
        let expected = Node::Eql(Box::new(Node::Int(2)), Box::new(Node::Int(3)));
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
