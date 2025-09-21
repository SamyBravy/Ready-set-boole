use crate::{boolean_evaluation::{eval_formula, ASTNode}, truth_table::print_truth_table};
mod adder;
mod boolean_evaluation;
mod gray_code;
mod multiplier;
mod negation_normal_form;
mod truth_table;

fn display_mathematical_formula(node: &ASTNode) {
	match node {
		ASTNode::Value(v) => print!("{}", match v {
			'0' => '⊥',
			'1' => '⊤',
			_ => *v,
		}),
		ASTNode::Op { operator, left, right } => {
			if let Some(l) = left.as_ref() {
				print!("(");
				display_mathematical_formula(l);
				print!(")");
			}
			print!(" {} ", match operator {
				'!' => '¬',
				'&' => '∧',
				'|' => '∨',
				'^' => '⊕',
				'>' => '→',
				'=' => '↔',
				_ => *operator,
			});
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
		println!("The product of {} and {} is: {}", a, b, multiplier::multiplier(a, b));
	}

	print_section("GRAY CODE");
	for n in [10, 25] {
		println!("The Gray code of {} is: {:b}", n, gray_code::gray_code(n));
	}

	print_section("BOOLEAN EVALUATION");
	let formulas = ["10&", "10|", "11>", "10=", "1011||=", "1011", "101111111||="];
	for f in formulas {
		if let Some(ast) = boolean_evaluation::build_ast(f) {
			print!("{} {{", f);
			display_mathematical_formula(&ast);
			println!("}}: {}", boolean_evaluation::eval_formula(f));
		}
		else
		{
			eval_formula(f);
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
		}
		else
		{
			print_truth_table(expr);
		}
	}

	print_section("NEGATION NORMAL FORM");
	let nnf_exprs = ["AB&!", "AB|!", "AB>", "A!!B!!!!>", "AB=", "AB|C&!", "1011", "101111111||="];
	for expr in nnf_exprs {
		if let Some(ast) = boolean_evaluation::build_ast(expr) {
			print!("NNF of {} {{", expr);
			display_mathematical_formula(&ast);
			let nnf = negation_normal_form::negation_normal_form(expr);
			print!("}}: {} {{", nnf);
			display_mathematical_formula(&boolean_evaluation::build_ast(&nnf).unwrap());
			println!(")");
		}
		else
		{
			negation_normal_form::negation_normal_form(expr);
		}
	}
}