use std::vec;

mod adder;
mod boolean_evaluation;
mod conjunctive_normal_form;
mod curve;
mod gray_code;
mod inverse_function;
mod multiplier;
mod negation_normal_form;
mod powerset;
mod sat;
mod set_evaluation;
mod truth_table;

fn display_mathematical_formula(node: &boolean_evaluation::ASTNode<char>) {
    match node {
        boolean_evaluation::ASTNode::Value(v) => print!(
            "{}",
            match v {
                '0' => '⊥',
                '1' => '⊤',
                _ => *v,
            }
        ),
        boolean_evaluation::ASTNode::Op {
            operator,
            left,
            right,
        } => {
            if let Some(l) = left.as_ref() {
                print!("(");
                display_mathematical_formula(l);
                print!(")");
            }
            print!(
                " {} ",
                match operator {
                    '!' => '¬',
                    '&' => '∧',
                    '|' => '∨',
                    '^' => '⊕',
                    '>' => '→',
                    '=' => '↔',
                    _ => *operator,
                }
            );
            print!("(");
            display_mathematical_formula(right);
            print!(")");
        }
    }
}

fn print_section(title: &str) {
    println!("\n\x1b[33m### {} ###\x1b[0m", title);
}

fn main() {
    print_section("ADDER");
    for (a, b) in [(5, 7), (15, 27)] {
        println!("The sum of {} and {} is: {}", a, b, adder::adder(a, b));
    }

    print_section("MULTIPLIER");
    for (a, b) in [(15, 27), (30, 40)] {
        println!(
            "The product of {} and {} is: {}",
            a,
            b,
            multiplier::multiplier(a, b)
        );
    }

    print_section("GRAY CODE");
    for n in [10, 25] {
        println!("The Gray code of {} is: {:b}", n, gray_code::gray_code(n));
    }

    print_section("BOOLEAN EVALUATION");
    let formulas = [
        "10&",
        "10|",
        "11>",
        "10=",
        "1011||=",
        "1011",
        "101111111||=",
    ];
    for f in formulas {
        if let Some(ast) = boolean_evaluation::build_ast(f) {
            print!("{} {{", f);
            display_mathematical_formula(&ast);
            println!("}}: {}", boolean_evaluation::eval_formula(f));
        } else {
            boolean_evaluation::eval_formula(f);
        }
    }

    print_section("TRUTH TABLE");
    let truth_exprs = ["AB&C|", "AB&A!B!&|", "A!B|B!A|&", "1011", "101111111||="];
    for expr in truth_exprs {
        if let Some(ast) = boolean_evaluation::build_ast(expr) {
            print!("Truth table for {} {{", expr);
            display_mathematical_formula(&ast);
            println!("}}:");
            truth_table::print_truth_table(expr);
        } else {
            truth_table::print_truth_table(expr);
        }
    }

    print_section("NEGATION NORMAL FORM");
    let nnf_exprs = [
        "AB&!",
        "AB|!",
        "AB>",
        "A!!B!!!!>",
        "AB=",
        "AB|C&!",
        "AB^",
        "1011",
        "101111111||=",
    ];
    for expr in nnf_exprs {
        if let Some(ast) = boolean_evaluation::build_ast(expr) {
            print!("NNF of {} {{", expr);
            display_mathematical_formula(&ast);
            let nnf = negation_normal_form::negation_normal_form(expr);
            print!("}}: {} {{", nnf);
            display_mathematical_formula(&boolean_evaluation::build_ast(&nnf).unwrap());
            println!("}}");
        } else {
            negation_normal_form::negation_normal_form(expr);
        }
    }

    print_section("CONJUNCTIVE NORMAL FORM");
    let cnf_exprs = [
        "AB&!",
        "AB|!",
        "AB|C&",
        "AB|C|D|",
        "AB&C&D&",
        "AB&!C!|",
        "AB|!C!&",
        "AB|C&DEF|&|",
        "1011",
        "101111111||=",
    ];
    for expr in cnf_exprs {
        if let Some(ast) = boolean_evaluation::build_ast(expr) {
            print!("CNF of {} {{", expr);
            display_mathematical_formula(&ast);
            let nnf = conjunctive_normal_form::conjunctive_normal_form(expr);
            print!("}}: {} {{", nnf);
            display_mathematical_formula(&boolean_evaluation::build_ast(&nnf).unwrap());
            println!("}}");
        } else {
            conjunctive_normal_form::conjunctive_normal_form(expr);
        }
    }

    print_section("SAT");
    let sat_exprs = ["AB|", "AB&", "AA!&", "AA^", "1011", "101111111||="];
    for expr in sat_exprs {
        if let Some(ast) = boolean_evaluation::build_ast(expr) {
            print!("SAT of {} {{", expr);
            display_mathematical_formula(&ast);
            let sat = sat::sat(expr);
            println!("}}: {}", sat);
        } else {
            sat::sat(expr);
        }
    }

    print_section("POWERSET");
    let sets: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![], vec![5, 10, 15, 20]];
    for set in sets {
        println!(
            "The powerset of {:?} is: {:?}",
            set,
            powerset::powerset(set.clone())
        );
    }

    print_section("SET EVALUATION");
    let sets_vec = vec![
        vec![vec![0, 1, 2], vec![0, 3, 4]],
        vec![vec![0, 1, 2], vec![3, 4, 5]],
        vec![vec![0, 1, 2]],
        vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
    ];
    let formulas = ["AB&", "AB|", "A!", "A!", "1011", "101111111||="];
    for (sets, formulas) in sets_vec.iter().zip(formulas.iter()) {
        if let Some(ast) = boolean_evaluation::build_ast(formulas) {
            print!("{} {{", formulas);
            display_mathematical_formula(&ast);
            let result = set_evaluation::eval_set(formulas, sets.clone());
            println!("}} with sets {:?} is: {:?}", sets, result);
        } else {
            set_evaluation::eval_set(formulas, sets.clone());
        }
    }

    print_section("CURVE");
    let points = [
        (u16::MIN, u16::MIN),
        (5, 10),
        (15, 27),
        (190, 470),
        (30000, 40000),
        (u16::MAX, u16::MAX),
    ];
    let mut results: Vec<f64> = Vec::new();
    for (x, y) in points {
        results.push(curve::map(x, y));
        println!(
            "Z-order map: ({}, {}) {{({:b}, {:b})}} is: {} {{{:b}}}",
            x,
            y,
            x,
            y,
            results.last().unwrap(),
            (results.last().unwrap() * (u32::MAX as f64)) as u32
        );
    }

    print_section("INVERSE FUNCTION");
    for n in results {
        let (x, y) = inverse_function::reverse_map(n);
        println!(
            "Inverse Z-order of {} {{{:b}}} is: ({}, {}) {{({:b}, {:b})}}",
            n,
            (n * (u32::MAX as f64)) as u32,
            x,
            y,
            x,
            y
        );
    }
	print!("Inverse Z-order of an out-of-bounds value: ");
	inverse_function::reverse_map(1.5);
}

#[cfg(test)]
mod tests;
