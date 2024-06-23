use ready_set_boole::adder::adder;
use ready_set_boole::boolean_eval::eval_formula;
use ready_set_boole::conjuctive_normal_form::conjunctive_normal_form;
use ready_set_boole::gray_code::gray_code;
use ready_set_boole::multiplier::multiplier;
use ready_set_boole::negation_normal_form::negation_normal_form;
use ready_set_boole::truth_table::print_truth_table;

fn main() {
    println!("3 + 4 = {}", adder(3, 4));
    println!("3 * 4 = {}", multiplier(3, 4));
    println!("Gray code for 3: {}", gray_code(3));
    println!("01&1| = {}", eval_formula("01&1|"));
    print_truth_table("AB&C|");
    println!("AB|! = {}", negation_normal_form("AB|!"));
    println!("AB|!C!& = {}", conjunctive_normal_form("AB|!C!&"));
}
