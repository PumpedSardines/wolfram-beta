use super::{parse as full_parse, LexerToken, NodeToken, ParserError};

/// Parses parentheses
pub(super) fn parse(tokens: &[LexerToken]) -> Result<Vec<NodeToken>, ParserError> {
    let mut node_tokens = vec![];

    let mut current_parsing = vec![];

    let mut parentheses_count = 0;
    for token in tokens {
        match token {
            LexerToken::LeftParenthesis => {
                parentheses_count += 1;
            }
            LexerToken::RightParenthesis => {
                parentheses_count -= 1;

                if parentheses_count < 0 {
                    return Err(ParserError::InvalidExpression);
                }

                if parentheses_count == 0 {
                    let node = full_parse(&current_parsing)?;
                    node_tokens.push(NodeToken::Node(node));
                    current_parsing.clear();
                }
            }
            _ => {
                if parentheses_count == 0 {
                    node_tokens.push(NodeToken::Token(token.clone()));
                } else {
                    current_parsing.push(token.clone());
                }
            }
        }
    }

    if parentheses_count > 0 {
        return Err(ParserError::InvalidExpression);
    }

    Ok(node_tokens)
}

#[cfg(test)]
mod tests {
    // TODO: Write tests, this function is a little bit tricky to test since it depends on the full_parse function. I will instead write tests for the full_parse function that uses parentheses::parse
}
