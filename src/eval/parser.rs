use super::lexer::Token;

#[derive(Debug, PartialEq)]
pub(super) enum Node {
    Int(i64),                  // integer
    Add(Box<Node>, Box<Node>), // +
    Sub(Box<Node>, Box<Node>), // -
}

// <expr> ::= <int> (("+" | "-") <int>)?
pub(super) fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (mut node, rest) = parse_int(tokens)?;

    if let Some(Token::Punct(p)) = rest.get(0) {
        match &**p {
            "+" => {
                let (rhs, _) = parse_int(&rest[1..])?;
                node = Node::Add(Box::new(node), Box::new(rhs));
            }
            "-" => {
                let (rhs, _) = parse_int(&rest[1..])?;
                node = Node::Sub(Box::new(node), Box::new(rhs));
            }
            _ => (),
        }
    }

    Ok(node)
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
    fn parses_addition() {
        // 2+3
        let tokens = vec![Token::Int(2), Token::Punct("+".to_string()), Token::Int(3)];
        let expected = Node::Add(
            Box::new(Node::Int(2)),
            Box::new(Node::Int(3)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_subtract() {
        // 4-1
        let tokens = vec![Token::Int(4), Token::Punct("-".to_string()), Token::Int(1)];
        let expected = Node::Sub(
            Box::new(Node::Int(4)),
            Box::new(Node::Int(1)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
