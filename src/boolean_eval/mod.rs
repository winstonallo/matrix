use std::collections::VecDeque;

enum ASTNode {
    Const(bool),
    Not(Box<ASTNode>),
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Xor(Box<ASTNode>, Box<ASTNode>),
    Implies(Box<ASTNode>, Box<ASTNode>),
    Equiv(Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    fn evaluate(&self) -> bool {
        match self {
            ASTNode::Const(val) => *val,
            ASTNode::Not(expr) => !expr.evaluate(),
            ASTNode::And(left, right) => left.evaluate() && right.evaluate(),
            ASTNode::Or(left, right) => left.evaluate() || right.evaluate(),
            ASTNode::Xor(left, right) => left.evaluate() != right.evaluate(),
            ASTNode::Implies(left, right) => !left.evaluate() || right.evaluate(),
            ASTNode::Equiv(left, right) => left.evaluate() == right.evaluate(),
        }
    }
}

fn rpn_to_ast(expression: &str) -> Option<ASTNode> {
    let mut stack: VecDeque<ASTNode> = VecDeque::new();

    for char in expression.chars() {
        match char {
            '0' => stack.push_back(ASTNode::Const(false)),
            '1' => stack.push_back(ASTNode::Const(true)),
            '!' => {
                if let Some(expr) = stack.pop_back() {
                    stack.push_back(ASTNode::Not(Box::new(expr)));
                } else {
                    return None;
                }
            }
            '&' | '|' | '^' | '>' | '=' => {
                if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                    let node = match char {
                        '&' => ASTNode::And(Box::new(left), Box::new(right)),
                        '|' => ASTNode::Or(Box::new(left), Box::new(right)),
                        '^' => ASTNode::Xor(Box::new(left), Box::new(right)),
                        '>' => ASTNode::Implies(Box::new(left), Box::new(right)),
                        '=' => ASTNode::Equiv(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                    stack.push_back(node);
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    if stack.len() == 1 {
        stack.pop_back()
    } else {
        None
    }
}

pub fn eval_formula(expression: &str) -> bool {
    match rpn_to_ast(expression) {
        Some(ast) => ast.evaluate(),
        None => {
            println!("Invalid expression: {}", expression);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(eval_formula("0"), false);
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("0!"), true);
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("01&"), false);
        assert_eq!(eval_formula("01|"), true);
        assert_eq!(eval_formula("01^"), true);
        assert_eq!(eval_formula("01>"), true);
        assert_eq!(eval_formula("01="), false);
        assert_eq!(eval_formula("0!1&"), true);
        assert_eq!(eval_formula("0!1|"), true);
        assert_eq!(eval_formula("0!1^"), false);
        assert_eq!(eval_formula("0!1>"), true);
        assert_eq!(eval_formula("0!1="), true);
        assert_eq!(eval_formula("0!1&0&"), false);
        assert_eq!(eval_formula("0!1&1&"), true);
        assert_eq!(eval_formula("0!1|0|"), true);
        assert_eq!(eval_formula("0!1|1|"), true);
        assert_eq!(eval_formula("0!1^0^"), false);
        assert_eq!(eval_formula("0!1^1^"), true);
        assert_eq!(eval_formula("0!1>0>"), false);
        assert_eq!(eval_formula("0!1>1>"), true);
        assert_eq!(eval_formula("0!1=0="), false);
        assert_eq!(eval_formula("0!1=1="), true);
        assert_eq!(eval_formula("0!1&0|"), true);
        assert_eq!(eval_formula("0!1|0&"), false);
        assert_eq!(eval_formula("0!1&0^"), true);
        assert_eq!(eval_formula("0!1^0&"), false);
        assert_eq!(eval_formula("0!1&0>"), false);
        assert_eq!(eval_formula("0!1>0&"), false);
        assert_eq!(eval_formula("0!1&0="), false);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(eval_formula("0!1&0"), false);
        assert_eq!(eval_formula("0!1|0"), false);
        assert_eq!(eval_formula("0!1^0"), false);
        assert_eq!(eval_formula("0!1>0"), false);
        assert_eq!(eval_formula("0!1=0"), false);
        assert_eq!(eval_formula("0!1&0!"), false);
        assert_eq!(eval_formula("0!1|0!"), false);
        assert_eq!(eval_formula("0!1^0!"), false);
        assert_eq!(eval_formula("0!1>0!"), false);
        assert_eq!(eval_formula("0!1=0!"), false);
        assert_eq!(eval_formula("0!1&0!1"), false);
        assert_eq!(eval_formula("0!1|0!1"), false);
        assert_eq!(eval_formula("0!1^0!1"), false);
        assert_eq!(eval_formula("0!1>0!1"), false);
        assert_eq!(eval_formula("0!1=0!1"), false);
    }
}
