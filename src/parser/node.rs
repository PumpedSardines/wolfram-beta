use crate::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Number(Number),
    Variable(String),
    PiConstant,
    EConstant,
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Exp(Box<Node>),
    Log(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Tan(Box<Node>),
    Sqrt(Box<Node>),
    Neg(Box<Node>),
}
