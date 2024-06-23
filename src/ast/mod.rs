use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Const(char),
    Not(Box<ASTNode>),
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Implies(Box<ASTNode>, Box<ASTNode>),
    Equiv(Box<ASTNode>, Box<ASTNode>),
}

pub fn rpn_to_ast(expression: &str) -> Option<ASTNode> {
    let mut stack: VecDeque<ASTNode> = VecDeque::new();

    for char in expression.chars() {
        match char {
            'A'..='Z' => stack.push_back(ASTNode::Const(char)),
            '!' => {
                if let Some(expr) = stack.pop_back() {
                    stack.push_back(ASTNode::Not(Box::new(expr)));
                } else {
                    return None;
                }
            }
            '&' | '|' | '>' | '=' => {
                if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                    let node = match char {
                        '&' => ASTNode::And(Box::new(left), Box::new(right)),
                        '|' => ASTNode::Or(Box::new(left), Box::new(right)),
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

pub fn to_nnf(node: ASTNode) -> ASTNode {
    match node {
        ASTNode::Not(boxed) => match *boxed {
            ASTNode::Not(inner) => to_nnf(*inner),
            ASTNode::And(left, right) => {
                let left = ASTNode::Not(Box::new(*left));
                let right = ASTNode::Not(Box::new(*right));
                ASTNode::Or(Box::new(to_nnf(left)), Box::new(to_nnf(right)))
            }
            ASTNode::Or(left, right) => {
                let left = ASTNode::Not(Box::new(*left));
                let right = ASTNode::Not(Box::new(*right));
                ASTNode::And(Box::new(to_nnf(left)), Box::new(to_nnf(right)))
            }
            ASTNode::Implies(left, right) => {
                let left = ASTNode::Not(Box::new(*left));
                ASTNode::And(Box::new(to_nnf(left)), Box::new(to_nnf(*right)))
            }
            ASTNode::Equiv(left, right) => {
                let left_and_right =
                    ASTNode::And(Box::new(*left.clone()), Box::new(*right.clone()));
                let not_left_and_not_right = ASTNode::And(
                    Box::new(ASTNode::Not(Box::new(*left))),
                    Box::new(ASTNode::Not(Box::new(*right))),
                );
                ASTNode::Or(
                    Box::new(to_nnf(ASTNode::Not(Box::new(left_and_right)))),
                    Box::new(to_nnf(ASTNode::Not(Box::new(not_left_and_not_right)))),
                )
            }
            ASTNode::Const(c) => ASTNode::Not(Box::new(ASTNode::Const(c))),
        },
        ASTNode::And(left, right) => {
            ASTNode::And(Box::new(to_nnf(*left)), Box::new(to_nnf(*right)))
        }
        ASTNode::Or(left, right) => ASTNode::Or(Box::new(to_nnf(*left)), Box::new(to_nnf(*right))),
        ASTNode::Implies(left, right) => {
            let left = ASTNode::Not(Box::new(*left));
            ASTNode::Or(Box::new(to_nnf(left)), Box::new(to_nnf(*right)))
        }
        ASTNode::Equiv(left, right) => {
            let left_and_right = ASTNode::And(Box::new(*left.clone()), Box::new(*right.clone()));
            let not_left_and_not_right = ASTNode::And(
                Box::new(ASTNode::Not(Box::new(*left))),
                Box::new(ASTNode::Not(Box::new(*right))),
            );
            ASTNode::Or(
                Box::new(to_nnf(left_and_right)),
                Box::new(to_nnf(not_left_and_not_right)),
            )
        }
        node => node,
    }
}

pub fn ast_to_string(node: ASTNode) -> String {
    match node {
        ASTNode::Const(c) => c.to_string(),
        ASTNode::Not(expr) => format!("{}!", ast_to_string(*expr)),
        ASTNode::And(left, right) => format!("{}{}&", ast_to_string(*left), ast_to_string(*right)),
        ASTNode::Or(left, right) => format!("{}{}|", ast_to_string(*left), ast_to_string(*right)),
        ASTNode::Implies(left, right) => {
            format!("{}{}>", ast_to_string(*left), ast_to_string(*right))
        }
        ASTNode::Equiv(left, right) => {
            format!("{}{}=", ast_to_string(*left), ast_to_string(*right))
        }
    }
}
