use ready_set_boole::adder;
use ready_set_boole::boolean_eval;
use ready_set_boole::gray_code;
use ready_set_boole::multiplier;

fn main() {
    println!("3 + 4 = {}", adder::adder(3, 4));
    println!("3 * 4 = {}", multiplier::multiplier(3, 4));
    println!("Gray code for 3: {}", gray_code::gray_code(3));
    println!("101|& = {}", boolean_eval::eval_formula("101|&"));
}
