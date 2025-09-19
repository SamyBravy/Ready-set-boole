mod adder;
mod boolean_evaluation;
mod gray;
mod multiplier;
mod truth_table;

fn main() {
    let sum = adder::adder(4, 12);
    let mul = multiplier::multiplier(3, 17);
    println!("The sum is: {}\nThe mult is: {}", sum, mul);
    let g = gray::gray_code(80);
    println!("The gray code of 80 is: {}", g);
    println!("{}", boolean_evaluation::eval_formula("10&"));
    // false
    println!("{}", boolean_evaluation::eval_formula("10|"));
    // true
    println!("{}", boolean_evaluation::eval_formula("11>"));
    // true
    println!("{}", boolean_evaluation::eval_formula("10="));
    // false
    println!("{}", boolean_evaluation::eval_formula("1011"));
    // true
    println!("{}", boolean_evaluation::eval_formula("101111111||="));

    truth_table::print_truth_table("AB&C|");
}
