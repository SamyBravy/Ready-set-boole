use crate::boolean_evaluation::build_ast;
use crate::boolean_evaluation::eval_node;
use crate::truth_table::substitute_vars;
use crate::truth_table::create_dict;
use crate::truth_table::update_dict;

pub fn sat(formula: &str) -> bool
{
	if build_ast(formula).is_none()
	{
		println!("Error in formula");
		return false;
	}

	let mut dict = create_dict(formula);
	if dict.is_empty()
	{
		println!("No variables in formula");
		return false;
	}

	for i in 0..(1 << dict.len())
	{
		update_dict(&mut dict, i);

		let new_formula = substitute_vars(&formula, &dict);
		let tree = build_ast(&new_formula).unwrap();
		if eval_node(&tree).unwrap()
		{
			return true;
		}
	}
	false
}