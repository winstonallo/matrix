use crate::ast::{ast_to_string, rpn_to_ast, to_nnf};

pub fn negation_normal_form(expression: &str) -> String {
    if let Some(ast) = rpn_to_ast(expression) {
        let nnf_ast = to_nnf(ast);
        ast_to_string(nnf_ast)
    } else {
        return "Invalid expression".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_normal_form() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn test_invalid_expression() {
        assert_eq!(negation_normal_form("ABBB"), "Invalid expression");
        assert_eq!(negation_normal_form(""), "Invalid expression");
        assert_eq!(negation_normal_form("!AB"), "Invalid expression");
    }
}
