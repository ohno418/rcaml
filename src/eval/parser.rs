use super::lexer::Token;

#[derive(Debug, PartialEq)]
pub(super) enum Node {
    Int(i64),                  // integer
    Add(Box<Node>, Box<Node>), // +
    Sub(Box<Node>, Box<Node>), // -
}

// <expr> ::= <int> (("+" | "-") <int>)*
pub(super) fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (mut node, mut rest) = parse_int(tokens)?;

    loop {
        match rest.get(0) {
            Some(Token::Punct(p)) => {
                match &**p {
                    "+" => {
                        let rhs;
                        (rhs, rest) = parse_int(&rest[1..])?;
                        node = Node::Add(Box::new(node), Box::new(rhs));
                    }
                    "-" => {
                        let rhs;
                        (rhs, rest) = parse_int(&rest[1..])?;
                        node = Node::Sub(Box::new(node), Box::new(rhs));
                    }
                    _ => break,
                }
            }
            _ => break,
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
    fn parses_add_sub() {
        // 2+3-4+5
        let tokens = vec![
            Token::Int(2),
            Token::Punct("+".to_string()),
            Token::Int(3),
            Token::Punct("-".to_string()),
            Token::Int(4),
            Token::Punct("+".to_string()),
            Token::Int(5),
        ];
        let expected = Node::Add(
            Box::new(Node::Sub(
                Box::new(Node::Add(
                    Box::new(Node::Int(2)),
                    Box::new(Node::Int(3)),
                )),
                Box::new(Node::Int(4)),
            )),
            Box::new(Node::Int(5)),
        );
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
