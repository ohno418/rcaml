use super::lexer::Token;

#[derive(Debug, PartialEq)]
pub(super) enum Node {
    Int(i64), // integer
}

pub(super) fn parse(tokens: &[Token]) -> Result<Node, String> {
    if tokens.len() != 1 {
        return Err("Only one token is expected".to_string());
    }

    match tokens.get(0) {
        Some(Token::Int(n)) => Ok(Node::Int(*n)),
        None => unreachable!(),
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
}
