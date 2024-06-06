use super::{LexerToken, Node, NodeToken, ParserError};

/// Parses the emdas in pemdas, aka everything except parentheses.
///
/// This expects that parentheses and functions have already been parsed.
pub(super) fn parse(node_tokens: &[NodeToken]) -> Result<Node, ParserError> {
    if node_tokens.len() == 0 {
        return Err(ParserError::BranchEvaluatedToNone);
    }

    if node_tokens.len() == 1 {
        return match &node_tokens[0] {
            NodeToken::Node(node) => Ok(node.clone()),
            NodeToken::Token(LexerToken::Number(number)) => Ok(Node::Number(number.clone())),
            NodeToken::Token(LexerToken::EConstant) => Ok(Node::EConstant),
            NodeToken::Token(LexerToken::PiConstant) => Ok(Node::PiConstant),
            NodeToken::Token(token) => Err(ParserError::InvalidToken(token.clone())),
        };
    }

    for i in (1..node_tokens.len()).rev() {
        if let NodeToken::Token(token) = &node_tokens[i] {
            match token {
                LexerToken::AddOperator => {
                    let left = parse(&node_tokens[..i])?;
                    let right = parse(&node_tokens[i + 1..])?;
                    return Ok(Node::Add(Box::new(left), Box::new(right)));
                }
                LexerToken::SubOperator => {
                    // This is a negation sub
                    if let Some(token) = node_tokens[i - 1].clone().token() {
                        if token.is_operator() {
                            continue;
                        }
                    }

                    let left = parse(&node_tokens[..i])?;
                    let right = parse(&node_tokens[i + 1..])?;
                    return Ok(Node::Sub(Box::new(left), Box::new(right)));
                }
                _ => {}
            }
        }
    }

    for i in (1..node_tokens.len()).rev() {
        if let NodeToken::Token(token) = &node_tokens[i] {
            match token {
                LexerToken::MulOperator => {
                    let left = parse(&node_tokens[..i])?;
                    let right = parse(&node_tokens[i + 1..])?;
                    return Ok(Node::Mul(Box::new(left), Box::new(right)));
                }
                LexerToken::DivOperator => {
                    let left = parse(&node_tokens[..i])?;
                    let right = parse(&node_tokens[i + 1..])?;
                    return Ok(Node::Div(Box::new(left), Box::new(right)));
                }
                _ => {}
            }
        }
    }

    if let NodeToken::Token(token) = &node_tokens[0] {
        if *token == LexerToken::SubOperator {
            let right = parse(&node_tokens[1..])?;
            return Ok(Node::Neg(Box::new(right)));
        }
    }

    for i in 1..node_tokens.len() {
        if let NodeToken::Token(token) = &node_tokens[i] {
            match token {
                LexerToken::PowOperator => {
                    let left = parse(&node_tokens[..i])?;
                    let right = parse(&node_tokens[i + 1..])?;
                    return Ok(Node::Pow(Box::new(left), Box::new(right)));
                }
                _ => {}
            }
        }
    }

    Err(ParserError::InvalidExpression)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Number;

    fn to_node_tokens(tokens: Vec<LexerToken>) -> Vec<NodeToken> {
        tokens.into_iter().map(NodeToken::Token).collect()
    }

    #[test]
    fn test_parse() {
        let tokens = to_node_tokens(vec![
            LexerToken::Number(Number::from_str("1").unwrap()),
            LexerToken::AddOperator,
            LexerToken::Number(Number::from_str("2").unwrap()),
        ]);

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Add(
                Box::new(Node::Number(Number::from_str("1").unwrap())),
                Box::new(Node::Number(Number::from_str("2").unwrap()))
            )
        );
    }

    #[test]
    fn test_parse_simple_pemdas() {
        let tokens = to_node_tokens(vec![
            LexerToken::Number(Number::from_str("1").unwrap()),
            LexerToken::AddOperator,
            LexerToken::Number(Number::from_str("2").unwrap()),
            LexerToken::MulOperator,
            LexerToken::Number(Number::from_str("3").unwrap()),
        ]);

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Add(
                Box::new(Node::Number(Number::from_str("1").unwrap())),
                Box::new(Node::Mul(
                    Box::new(Node::Number(Number::from_str("2").unwrap())),
                    Box::new(Node::Number(Number::from_str("3").unwrap()))
                ))
            )
        );
    }

    #[test]
    fn test_parse_simple_pemdas_pow() {
        let tokens = to_node_tokens(vec![
            LexerToken::Number(Number::from_str("7").unwrap()),
            LexerToken::MulOperator,
            LexerToken::Number(Number::from_str("8").unwrap()),
            LexerToken::AddOperator,
            LexerToken::Number(Number::from_str("1").unwrap()),
            LexerToken::PowOperator,
            LexerToken::Number(Number::from_str("2").unwrap()),
            LexerToken::PowOperator,
            LexerToken::Number(Number::from_str("3").unwrap()),
        ]);

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Add(
                Box::new(Node::Mul(
                    Box::new(Node::Number(Number::from_str("7").unwrap())),
                    Box::new(Node::Number(Number::from_str("8").unwrap()))
                )),
                Box::new(Node::Pow(
                    Box::new(Node::Number(Number::from_str("1").unwrap())),
                    Box::new(Node::Pow(
                        Box::new(Node::Number(Number::from_str("2").unwrap())),
                        Box::new(Node::Number(Number::from_str("3").unwrap()))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_parses_constants() {
        let tokens = to_node_tokens(vec![
            LexerToken::EConstant,
            LexerToken::AddOperator,
            LexerToken::PiConstant,
        ]);

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Add(Box::new(Node::EConstant), Box::new(Node::PiConstant))
        );
    }

    #[test]
    fn test_parses_negation() {
        let tokens = to_node_tokens(vec![
            LexerToken::SubOperator,
            LexerToken::EConstant,
            LexerToken::AddOperator,
            LexerToken::SubOperator,
            LexerToken::PiConstant,
            LexerToken::PowOperator,
            LexerToken::Number(Number::from_str("2").unwrap()),
        ]);

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Add(
                Box::new(Node::Neg(Box::new(Node::EConstant))),
                Box::new(Node::Neg(Box::new(Node::Pow(
                    Box::new(Node::PiConstant),
                    Box::new(Node::Number(Number::from_str("2").unwrap()))
                ),)))
            )
        );
    }
}
