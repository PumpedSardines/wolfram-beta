use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    value: f64,
}

#[derive(Debug, Error)]
pub enum NumberError {
    #[error("Parsing error")]
    ParsingError,
}

impl Number {
    pub fn from_str(input: &str) -> Result<Self, NumberError> {
        let value = input.parse().map_err(|_| NumberError::ParsingError)?;
        Ok(Number { value })
    }

    pub fn add(right: &Number, left: &Number) -> Number {
        Number {
            value: left.value + right.value,
        }
    }

    pub fn sub(right: &Number, left: &Number) -> Number {
        Number {
            value: left.value - right.value,
        }
    }

    pub fn mul(right: &Number, left: &Number) -> Number {
        Number {
            value: left.value * right.value,
        }
    }

    pub fn div(right: &Number, left: &Number) -> Number {
        Number {
            value: left.value / right.value,
        }
    }

    pub fn pow(right: &Number, left: &Number) -> Number {
        Number {
            value: f64::powf(left.value, right.value),
        }
    }
}
