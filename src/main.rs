use ready_set_boole::adder;
use ready_set_boole::boolean_eval;
use ready_set_boole::gray_code;
use ready_set_boole::multiplier;
use ready_set_boole::negation_normal_form;
use ready_set_boole::truth_table;

fn main() {
    println!("3 + 4 = {}", adder::adder(3, 4));
    println!("3 * 4 = {}", multiplier::multiplier(3, 4));
    println!("Gray code for 3: {}", gray_code::gray_code(3));
    println!("01&1| = {}", boolean_eval::eval_formula("01&1|"));
    truth_table::print_truth_table("AB&C|");
    println!(
        "AB|! = {}",
        negation_normal_form::negation_normal_form("AB|!")
    );
}
