use super::Node;

/// Recursively change e^x to exp(x)
pub fn parse(node: Node) -> Node {
    match node {
        Node::Pow(l, r) => {
            let l = *l;
            let r = *r;

            if let Node::EConstant = l {
                Node::Exp(Box::new(parse(r)))
            } else {
                Node::Pow(Box::new(parse(l)), Box::new(parse(r)))
            }
        }
        Node::Add(l, r) => Node::Add(Box::new(parse(*l)), Box::new(parse(*r))),
        Node::Sub(l, r) => Node::Sub(Box::new(parse(*l)), Box::new(parse(*r))),
        Node::Div(l, r) => Node::Div(Box::new(parse(*l)), Box::new(parse(*r))),
        Node::Mul(l, r) => Node::Mul(Box::new(parse(*l)), Box::new(parse(*r))),
        Node::Exp(n) => Node::Exp(Box::new(parse(*n))),
        Node::Log(n) => Node::Log(Box::new(parse(*n))),
        Node::Sin(n) => Node::Sin(Box::new(parse(*n))),
        Node::Cos(n) => Node::Cos(Box::new(parse(*n))),
        Node::Tan(n) => Node::Tan(Box::new(parse(*n))),
        Node::Sqrt(n) => Node::Sqrt(Box::new(parse(*n))),
        Node::Neg(n) => Node::Neg(Box::new(parse(*n))),
        Node::PiConstant => Node::PiConstant,
        Node::EConstant => Node::EConstant,
        Node::Number(n) => Node::Number(n),
        Node::Variable(v) => Node::Variable(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Number;

    #[test]
    fn test_parse_exp() {
        let node = Node::Pow(
            Box::new(Node::EConstant),
            Box::new(Node::Number(Number::from_str("2").unwrap())),
        );

        let node = parse(node);

        assert_eq!(
            node,
            Node::Exp(Box::new(Node::Number(Number::from_str("2").unwrap())))
        );
    }

    #[test]
    fn test_parse_exp_nested() {
        let node = Node::Pow(
            Box::new(Node::EConstant),
            Box::new(Node::Pow(
                Box::new(Node::EConstant),
                Box::new(Node::Number(Number::from_str("2").unwrap())),
            )),
        );

        let node = parse(node);

        assert_eq!(
            node,
            Node::Exp(Box::new(Node::Exp(Box::new(Node::Number(
                Number::from_str("2").unwrap()
            )))))
        );
    }
}
