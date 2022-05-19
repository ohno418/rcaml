use super::lexer::{KwKind, Token};

#[derive(Debug, PartialEq)]
pub(super) enum Node {
    Int(i64),                   // integer
    Add(Box<Node>, Box<Node>),  // +
    Sub(Box<Node>, Box<Node>),  // -
    Mul(Box<Node>, Box<Node>),  // *
    Var(String),                // variable
    Bind(Box<Node>, Box<Node>), // binding
}

// <expr> ::= <bind>
pub(super) fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (node, rest) = parse_bind(tokens)?;

    if rest.len() != 0 {
        return Err("Found extra token".to_string());
    }

    Ok(node)
}

// <bind> ::= "let" identifier ("=" <add>)
//          | <add>
fn parse_bind(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    if let Some(Token::Kw(KwKind::Let)) = tokens.get(0) {
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
            _ => return Err("Expected an identifier".to_string()),
        };

        let rhs;
        (rhs, rest) = parse_add(rest)?;

        let node = Node::Bind(Box::new(Node::Var(ident)), Box::new(rhs));
        return Ok((node, rest));
    }

    parse_add(tokens)
}

// <add> ::= <mul> (("+" | "-") <mul>)*
fn parse_add(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_mul(tokens)?;

    loop {
        match rest.get(0) {
            Some(Token::Punct(p)) => match &**p {
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
            },
            _ => break,
        }
    }

    Ok((node, rest))
}

// <mul> ::= <int> ("*" <int>)*
fn parse_mul(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_int(tokens)?;

    loop {
        match rest.get(0) {
            Some(Token::Punct(p)) => match &**p {
                "*" => {
                    let rhs;
                    (rhs, rest) = parse_int(&rest[1..])?;
                    node = Node::Mul(Box::new(node), Box::new(rhs));
                }
                _ => break,
            },
            _ => break,
        }
    }

    Ok((node, rest))
}

// <int> ::= number
fn parse_int(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    match tokens.get(0) {
        Some(Token::Int(int)) => Ok((Node::Int(*int), &tokens[1..])),
        _ => Err(format!(r#"Failed to parse "{:#?}" into integer"#, tokens)),
    }
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
        // 2+3*4-5
        let tokens = vec![
            Token::Int(2),
            Token::Punct("+".to_string()),
            Token::Int(3),
            Token::Punct("*".to_string()),
            Token::Int(4),
            Token::Punct("-".to_string()),
            Token::Int(5),
        ];
        let expected = Node::Sub(
            Box::new(Node::Add(
                Box::new(Node::Int(2)),
                Box::new(Node::Mul(Box::new(Node::Int(3)), Box::new(Node::Int(4)))),
            )),
            Box::new(Node::Int(5)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_variable_binding() {
        // let foo = 123
        let tokens = vec![
            Token::Kw(KwKind::Let),
            Token::Ident("foo".to_string()),
            Token::Punct("=".to_string()),
            Token::Int(123),
        ];
        let expected = Node::Bind(
            Box::new(Node::Var("foo".to_string())),
            Box::new(Node::Int(123)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
