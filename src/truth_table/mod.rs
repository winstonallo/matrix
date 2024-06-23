use std::collections::{HashMap, HashSet, VecDeque};

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
    fn evaluate(&self, vars: &std::collections::HashMap<char, bool>) -> bool {
        match self {
            ASTNode::Const(val) => vars[val],
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

pub fn print_truth_table(expression: &str) {
    let mut variables = HashSet::new();
    for char in expression.chars() {
        if char.is_ascii_uppercase() {
            variables.insert(char);
        }
    }

    let variables: Vec<char> = variables.into_iter().collect();
    let num_vars = variables.len();
    let num_combinations = 1 << num_vars;

    if let Some(ast) = rpn_to_ast(expression) {
        print!("| ");
        for var in &variables {
            print!("{} | ", var);
        }
        println!("= |$");

        print!("|");
        for _ in &variables {
            print!("---|");
        }
        println!("---|$");

        for combination_index in 0..num_combinations {
            let mut var_values = HashMap::new();
            for (bit_pos, var) in variables.iter().enumerate() {
                let val = (combination_index & (1 << bit_pos)) != 0;
                var_values.insert(*var, val);
            }
            let result = ast.evaluate(&var_values);
            print!("| ");
            for var in &variables {
                print!("{} | ", if var_values[var] { 1 } else { 0 });
            }
            println!("{} |$", if result { 1 } else { 0 });
        }
    } else {
        println!("Invalid expression");
    }
}
