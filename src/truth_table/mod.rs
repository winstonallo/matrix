use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum ASTNode {
    Const(char),
    Not(Box<ASTNode>),
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Xor(Box<ASTNode>, Box<ASTNode>),
    Implies(Box<ASTNode>, Box<ASTNode>),
    Equiv(Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    fn evaluate(&self, vars: &HashMap<char, bool>) -> bool {
        match self {
            ASTNode::Const(val) => *vars.get(val).unwrap_or(&false),
            ASTNode::Not(expr) => !expr.evaluate(vars),
            ASTNode::And(left, right) => left.evaluate(vars) && right.evaluate(vars),
            ASTNode::Or(left, right) => left.evaluate(vars) || right.evaluate(vars),
            ASTNode::Xor(left, right) => left.evaluate(vars) != right.evaluate(vars),
            ASTNode::Implies(left, right) => !left.evaluate(vars) || right.evaluate(vars),
            ASTNode::Equiv(left, right) => left.evaluate(vars) == right.evaluate(vars),
        }
    }
}

fn rpn_to_ast(expression: &str) -> Option<ASTNode> {
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
impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ASTNode::Const(a), ASTNode::Const(b)) => a == b,
            (ASTNode::Not(a), ASTNode::Not(b)) => a == b,
            (ASTNode::And(a1, a2), ASTNode::And(b1, b2))
            | (ASTNode::Or(a1, a2), ASTNode::Or(b1, b2))
            | (ASTNode::Xor(a1, a2), ASTNode::Xor(b1, b2))
            | (ASTNode::Implies(a1, a2), ASTNode::Implies(b1, b2))
            | (ASTNode::Equiv(a1, a2), ASTNode::Equiv(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

fn extract_variables(expression: &str) -> Vec<char> {
    let mut variables = HashSet::new();
    for char in expression.chars() {
        if char.is_ascii_uppercase() {
            variables.insert(char);
        }
    }
    let mut variables: Vec<char> = variables.into_iter().collect();
    variables.sort();
    variables
}

fn generate_var_combinations(variables: &[char]) -> Vec<HashMap<char, bool>> {
    let num_vars = variables.len();
    let num_combinations = 1 << num_vars;
    let mut combinations = Vec::new();

    for combination_index in 0..num_combinations {
        let mut var_values = HashMap::new();
        for (bit_pos, var) in variables.iter().enumerate() {
            let val = (combination_index >> (num_vars - 1 - bit_pos)) & 1 != 0;
            var_values.insert(*var, val);
        }
        combinations.push(var_values);
    }
    combinations
}

fn print_header(variables: &[char]) {
    print!("| ");
    for var in variables {
        print!("{} | ", var);
    }
    println!("= |$");

    print!("|");
    for _ in variables {
        print!("---|");
    }
    println!("---|$");
}

fn print_truth_table_rows(vars: &[char], ast: &ASTNode, combs: Vec<HashMap<char, bool>>) {
    for var_values in combs {
        let result = ast.evaluate(&var_values);
        print!("| ");
        for var in vars {
            print!("{} | ", if var_values[var] { 1 } else { 0 });
        }
        println!("{} |$", if result { 1 } else { 0 });
    }
}

pub fn print_truth_table(expression: &str) {
    let variables = extract_variables(expression);
    let combinations = generate_var_combinations(&variables);

    if let Some(ast) = rpn_to_ast(expression) {
        print_header(&variables);
        print_truth_table_rows(&variables, &ast, combinations);
    } else {
        println!("Invalid expression");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn_to_ast() {
        let expression = "AB&C|";

        let ast = rpn_to_ast(expression).unwrap();
        let expected_ast = ASTNode::Or(
            Box::new(ASTNode::And(
                Box::new(ASTNode::Const('A')),
                Box::new(ASTNode::Const('B')),
            )),
            Box::new(ASTNode::Const('C')),
        );
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_extract_variables() {
        let expression = "AB&C|";
        let variables = extract_variables(expression);
        let expected_vars = vec!['A', 'B', 'C'];
        assert_eq!(variables, expected_vars);
    }
}
