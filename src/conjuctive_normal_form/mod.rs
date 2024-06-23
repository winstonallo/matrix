use crate::ast::{ast_to_string, rpn_to_ast, to_nnf, ASTNode};

fn distribute_or(node: ASTNode) -> ASTNode {
    match node {
        ASTNode::Or(left, right) => match (*left, *right) {
            (ASTNode::And(left1, right1), right2) => {
                let left_or = ASTNode::Or(Box::new(*left1), Box::new(right2.clone()));
                let right_or = ASTNode::Or(Box::new(*right1), Box::new(right2));
                ASTNode::And(
                    Box::new(distribute_or(left_or)),
                    Box::new(distribute_or(right_or)),
                )
            }
            (left2, ASTNode::And(left3, right3)) => {
                let left_or = ASTNode::Or(Box::new(left2.clone()), Box::new(*left3));
                let right_or = ASTNode::Or(Box::new(left2), Box::new(*right3));
                ASTNode::And(
                    Box::new(distribute_or(left_or)),
                    Box::new(distribute_or(right_or)),
                )
            }
            (left, right) => ASTNode::Or(
                Box::new(distribute_or(left)),
                Box::new(distribute_or(right)),
            ),
        },
        ASTNode::And(left, right) => ASTNode::And(
            Box::new(distribute_or(*left)),
            Box::new(distribute_or(*right)),
        ),
        node => node,
    }
}

fn to_cnf(node: ASTNode) -> ASTNode {
    match node {
        ASTNode::And(left, right) => {
            ASTNode::And(Box::new(to_cnf(*left)), Box::new(to_cnf(*right)))
        }
        ASTNode::Or(left, right) => distribute_or(ASTNode::Or(
            Box::new(to_cnf(*left)),
            Box::new(to_cnf(*right)),
        )),
        node => node,
    }
}

pub fn conjunctive_normal_form(expression: &str) -> String {
    let ast = rpn_to_ast(expression).unwrap();
    let nnf_ast = to_nnf(ast);
    let cnf_ast = to_cnf(nnf_ast);

    ast_to_string(cnf_ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnf_conversion() {
        let expression = "AB|!C!&";
        let cnf = conjunctive_normal_form(expression);
        assert_eq!(cnf, "A!B!C!&&");
    }
}
