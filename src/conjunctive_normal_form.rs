use crate::boolean_evaluation::ASTNode;
use crate::boolean_evaluation::build_ast;
use crate::negation_normal_form::tree_to_almost_nnf;
use crate::negation_normal_form::tree_to_string;
use std::mem;

pub fn tree_to_almost_cnf(node: &mut ASTNode, modified: &mut bool)
{
	match node {
		ASTNode::Op { operator, left, right } => {
			if *operator == '|'
			{
				if let ASTNode::Op { operator: '&', left: right_left, right: right_right } = &mut **right {
					*node = ASTNode::Op {
						operator: '&',
						left: Some(Box::new(ASTNode::Op {
							operator: '|',
							left: left.clone(),
							right: right_left.take().unwrap()
						})),
						right: Box::new(ASTNode::Op {
							operator: '|',
							left: left.take(),
							right: Box::new(mem::replace(right_right, ASTNode::Value('\0')))
						})
					};

					*modified = true;
					let ASTNode::Op { left, right, .. } = node else { return; };
					if let Some(l) = left.as_mut() {
						tree_to_almost_cnf(l, modified);
					}
					tree_to_almost_cnf(right, modified);
					return;
				}

				if let Some(left_node) = left.as_mut() {
					if let ASTNode::Op { operator: '&', left: left_left, right: left_right } = &mut **left_node {
						*node = ASTNode::Op {
							operator: '&',
							left: Some(Box::new(ASTNode::Op {
								operator: '|',
								left: Some(right.clone()),
								right: left_left.take().unwrap()
							})),
							right: Box::new(ASTNode::Op {
								operator: '|',
								left: Some(Box::new(mem::replace(right, ASTNode::Value('\0')))),
								right: Box::new(mem::replace(left_right, ASTNode::Value('\0')))
							})
						};
						*modified = true;
					}
				}
			}

			let ASTNode::Op { left, right, .. } = node else { return; };
			if let Some(l) = left.as_mut() {
				tree_to_almost_cnf(l, modified);
			}
			tree_to_almost_cnf(right, modified);
		},
		_ => { }
	}
}

#[allow(non_snake_case)]
pub fn conjunctive_normal_form(formula: &str) -> String
{
	if formula.contains('1') || formula.contains('0')
	{
		println!("Formula contains constants (0 or 1), cannot convert to CNF");
		return String::new();
	}
	let mut tree = match build_ast(formula) {
		Some(ast) => ast,
		None => {
			println!("Error in formula");
			return String::new();
		}
	};

	let mut modified = true;
	while modified
	{
		modified = false;
		tree_to_almost_nnf(&mut tree, &mut modified);
	}

	modified = true;
	while modified
	{
		modified = false;
		tree_to_almost_cnf(&mut tree, &mut modified);
	}

	tree_to_string(&tree)
}