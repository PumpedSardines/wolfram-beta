use thiserror::Error;

use crate::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerToken {
    Variable(String),
    Number(Number),
    Constant(String),
    AddOperator,
    SubOperator,
    MulOperator,
    DivOperator,
    PowOperator,
    LeftParenthesis,
    RightParenthesis,
    LogFunction,
    SinFunction,
    CosFunction,
    TanFunction,
    SqrtFunction,
    PiConstant,
    EConstant,
}

impl LexerToken {
    pub fn is_operator(&self) -> bool {
        match self {
            LexerToken::AddOperator
            | LexerToken::SubOperator
            | LexerToken::MulOperator
            | LexerToken::DivOperator
            | LexerToken::PowOperator => true,
            _ => false,
        }
    }
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
    #[error("Invalid character: {0}")]
    InvalidNumber(String),
}

#[derive(Debug, PartialEq)]
enum ParsingType {
    Number,
    Variable,
}

fn add_current(
    tokens: &mut Vec<LexerToken>,
    current_parsing: &mut String,
    current_parsing_type: &mut Option<ParsingType>,
) -> Result<(), LexerError> {
    if current_parsing.is_empty() {
        *current_parsing_type = None;
        return Ok(());
    }

    if *current_parsing_type == Some(ParsingType::Variable) {
        if let Some(token) = variable_to_token(&current_parsing) {
            tokens.push(token);
        } else {
            tokens.push(LexerToken::Variable(current_parsing.clone()));
        }
        current_parsing.clear();
        *current_parsing_type = None;
    } else if *current_parsing_type == Some(ParsingType::Number) {
        tokens.push(LexerToken::Number(
            Number::from_str(&current_parsing)
                .map_err(|_| LexerError::InvalidNumber(current_parsing.clone()))?,
        ));
        current_parsing.clear();
        *current_parsing_type = None;
    }

    return Ok(());
}

fn variable_to_token(s: &str) -> Option<LexerToken> {
    match s {
        "ln" => Some(LexerToken::LogFunction),
        "sin" => Some(LexerToken::SinFunction),
        "cos" => Some(LexerToken::CosFunction),
        "tan" => Some(LexerToken::TanFunction),
        "sqrt" => Some(LexerToken::SqrtFunction),
        "pi" => Some(LexerToken::PiConstant),
        "e" => Some(LexerToken::EConstant),
        _ => None,
    }
}

fn operator_to_token(c: char) -> Option<LexerToken> {
    match c {
        '+' => Some(LexerToken::AddOperator),
        '-' => Some(LexerToken::SubOperator),
        '*' => Some(LexerToken::MulOperator),
        '/' => Some(LexerToken::DivOperator),
        '^' => Some(LexerToken::PowOperator),
        '(' => Some(LexerToken::LeftParenthesis),
        ')' => Some(LexerToken::RightParenthesis),
        _ => None,
    }
}

/// Tokenizes input string into a vector of LexerToken
pub fn tokenize(input: &str) -> Result<Vec<LexerToken>, LexerError> {
    let mut tokens = vec![];

    let mut current_parsing_type: Option<ParsingType> = None;
    let mut current_parsing = String::new();

    for char in input.chars() {
        if char.is_whitespace() {
            add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;
            continue;
        }

        if char.is_alphabetic() && current_parsing_type != Some(ParsingType::Variable) {
            add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;
            current_parsing_type = Some(ParsingType::Variable);
        } else if (char.is_numeric() || char == '.')
            && current_parsing_type != Some(ParsingType::Number)
        {
            add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;
            current_parsing_type = Some(ParsingType::Number);
        } else if operator_to_token(char).is_some() {
            add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;
        }

        if current_parsing_type == Some(ParsingType::Variable) {
            current_parsing.push(char);
            continue;
        }

        if current_parsing_type == Some(ParsingType::Number) {
            current_parsing.push(char);
            continue;
        }

        if let Some(token) = operator_to_token(char) {
            add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;
            tokens.push(token);
            continue;
        } else {
            return Err(LexerError::InvalidCharacter(char));
        }
    }
    add_current(&mut tokens, &mut current_parsing, &mut current_parsing_type)?;

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_operators() {
        let input = "+-*^+/";
        let tokens = tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                LexerToken::AddOperator,
                LexerToken::SubOperator,
                LexerToken::MulOperator,
                LexerToken::PowOperator,
                LexerToken::AddOperator,
                LexerToken::DivOperator
            ]
        );
    }

    #[test]
    fn test_tokenize_variables() {
        let input = "x y+test";
        let tokens = tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                LexerToken::Variable("x".to_string()),
                LexerToken::Variable("y".to_string()),
                LexerToken::AddOperator,
                LexerToken::Variable("test".to_string())
            ]
        );
    }

    #[test]
    fn test_tokenize_number() {
        let input = "x 10+x1.50";
        let tokens = tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                LexerToken::Variable("x".to_string()),
                LexerToken::Number(Number::from_str("10").unwrap()),
                LexerToken::AddOperator,
                LexerToken::Variable("x".to_string()),
                LexerToken::Number(Number::from_str("1.5").unwrap()),
            ]
        );
    }

    #[test]
    fn test_convert_known_strings() {
        let input = "sin(x) - cos(pi)";
        let tokens = tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                LexerToken::SinFunction,
                LexerToken::LeftParenthesis,
                LexerToken::Variable("x".to_string()),
                LexerToken::RightParenthesis,
                LexerToken::SubOperator,
                LexerToken::CosFunction,
                LexerToken::LeftParenthesis,
                LexerToken::PiConstant,
                LexerToken::RightParenthesis,
            ]
        );
    }
}
