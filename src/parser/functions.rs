use super::{LexerToken, Node, NodeToken, ParserError};

/// Parses function calls, aka sqrt, sin, cos etc.
pub(super) fn parse(node_tokens: &[NodeToken]) -> Result<Vec<NodeToken>, ParserError> {
    let mut output = vec![];

    let mut skip_next = false;
    for i in 0..(node_tokens.len() - 1) {
        use LexerToken as LT;
        use NodeToken as NT;

        match node_tokens[i] {
            NT::Token(LT::SqrtFunction) => {
                if let Some(node) = node_tokens[i + 1].clone().node() {
                    skip_next = true;
                    output.push(NT::Node(Node::Sqrt(Box::new(node))));
                } else {
                    return Err(ParserError::InvalidFunctionCall);
                }
            }
            NT::Token(LT::SinFunction) => {
                if let Some(node) = node_tokens[i + 1].clone().node() {
                    skip_next = true;
                    output.push(NT::Node(Node::Sin(Box::new(node))));
                } else {
                    return Err(ParserError::InvalidFunctionCall);
                }
            }
            NT::Token(LT::CosFunction) => {
                if let Some(node) = node_tokens[i + 1].clone().node() {
                    skip_next = true;
                    output.push(NT::Node(Node::Cos(Box::new(node))));
                } else {
                    return Err(ParserError::InvalidFunctionCall);
                }
            }
            NT::Token(LT::TanFunction) => {
                if let Some(node) = node_tokens[i + 1].clone().node() {
                    skip_next = true;
                    output.push(NT::Node(Node::Tan(Box::new(node))));
                } else {
                    return Err(ParserError::InvalidFunctionCall);
                }
            }
            NT::Token(LT::LogFunction) => {
                if let Some(node) = node_tokens[i + 1].clone().node() {
                    skip_next = true;
                    output.push(NT::Node(Node::Log(Box::new(node))));
                } else {
                    return Err(ParserError::InvalidFunctionCall);
                }
            }
            _ => {
                if skip_next {
                    skip_next = false;
                    continue;
                }
                output.push(node_tokens[i].clone());
            }
        }
    }

    if !skip_next {
        output.push(node_tokens[node_tokens.len() - 1].clone());
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Number;

    #[test]
    fn test_parse_functions() {
        let tokens = vec![
            NodeToken::Token(LexerToken::SqrtFunction),
            NodeToken::Node(Node::Number(Number::from_str("4").unwrap())),
        ];

        let node_tokens = parse(&tokens).unwrap();

        assert_eq!(
            node_tokens,
            vec![NodeToken::Node(Node::Sqrt(Box::new(Node::Number(
                Number::from_str("4").unwrap()
            ))))]
        );
    }

    #[test]
    fn test_outputs_the_same_witout_functions() {
        let tokens = vec![
            NodeToken::Token(LexerToken::AddOperator),
            NodeToken::Token(LexerToken::AddOperator),
        ];

        let node_tokens = parse(&tokens).unwrap();

        assert_eq!(
            node_tokens,
            vec![
                NodeToken::Token(LexerToken::AddOperator),
                NodeToken::Token(LexerToken::AddOperator)
            ]
        );
    }
}
