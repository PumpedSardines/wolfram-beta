use crate::lexer::LexerToken;
use thiserror::Error;

mod node;
pub use node::Node;

mod emdas;
mod exp;
mod functions;
mod parentheses;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid token")]
    InvalidToken(LexerToken),
    #[error("Invalid expression")]
    InvalidExpression,
    #[error("Invalid function call")]
    InvalidFunctionCall,
    #[error("Branch evaluated to None")]
    BranchEvaluatedToNone,
}

/// Parses the tokens into a Node
///
/// This function is pretty slow, but since the tree only needs to be built once, it's not a big deal.
pub fn parse(tokens: &Vec<LexerToken>) -> Result<Node, ParserError> {
    let node_tokens = parentheses::parse(tokens)?;
    let node_tokens = functions::parse(&node_tokens)?;
    let node = emdas::parse(&node_tokens)?;
    let node = exp::parse(node);

    Ok(node)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeToken {
    Token(LexerToken),
    Node(Node),
}

impl NodeToken {
    pub fn node(self) -> Option<Node> {
        match self {
            NodeToken::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn token(self) -> Option<LexerToken> {
        match self {
            NodeToken::Token(token) => Some(token),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Number;

    #[test]
    fn test_parse_parentheses_pemdas() {
        let tokens = vec![
            LexerToken::Number(Number::from_str("7").unwrap()),
            LexerToken::MulOperator,
            LexerToken::LeftParenthesis,
            LexerToken::Number(Number::from_str("8").unwrap()),
            LexerToken::AddOperator,
            LexerToken::Number(Number::from_str("1").unwrap()),
            LexerToken::RightParenthesis,
        ];

        let node = parse(&tokens).unwrap();

        assert_eq!(
            node,
            Node::Mul(
                Box::new(Node::Number(Number::from_str("7").unwrap())),
                Box::new(Node::Add(
                    Box::new(Node::Number(Number::from_str("8").unwrap())),
                    Box::new(Node::Number(Number::from_str("1").unwrap()))
                )),
            )
        );
    }
}
