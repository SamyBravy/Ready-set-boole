use crate::boolean_evaluation::ASTNode;
use crate::boolean_evaluation::build_ast;
use std::mem;

pub fn tree_to_string(node: &ASTNode) -> String
{
	match node {
		ASTNode::Value(v) => v.to_string(),
        ASTNode::Op { operator, left, right } => {
            let mut res = String::new();

			if *operator == '&' || *operator == '|'
			{
            	res.push_str(&tree_to_string(right));
			}
            if let Some(l) = left.as_ref() {
                res.push_str(&tree_to_string(l));
            }
			if *operator != '&' && *operator != '|'
			{
				res.push_str(&tree_to_string(right));
			}
            res.push_str(&operator.to_string());

            res
        },
		_ => { String::new() }
	}
}

pub fn tree_to_almost_nnf(node: &mut ASTNode, modified: &mut bool)
{
	match node {
		ASTNode::Op { operator, left, right } => {
			match operator {
				'>' => {
					*node = ASTNode::Op {
						operator: '|',
						left: Some(Box::new(ASTNode::Op {
							operator: '!',
							left: None,
							right: left.take().unwrap(),
						})),
						right: Box::new(mem::replace(right, ASTNode::Value('\0'))),
					};
					*modified = true;
				},
				'=' => {
					*node = ASTNode::Op {
						operator: '&',
						left: Some(Box::new(ASTNode::Op {
							operator: '>',
							left: left.clone(),
							right: right.clone()
						})),
						right: Box::new(ASTNode::Op {
							operator: '>',
							left: Some(Box::new(mem::replace(right, ASTNode::Value('\0')))),
							right: left.take().unwrap()
						})
					};
					*modified = true;
				},
				'!' => {
					match &mut **right {
						ASTNode::Op { operator: '!', right: right_right, .. } => {
							*node = mem::replace(right_right, ASTNode::Value('\0'));
							*modified = true;
							tree_to_almost_nnf(node, modified);
							return ;
						},
						ASTNode::Op { operator, right: right_right, left: right_left }
							if *operator == '|' || *operator == '&' => {
								*node = ASTNode::Op {
									operator: if *operator == '|' { '&' } else { '|' },
									left: Some(Box::new(ASTNode::Op {
										operator: '!',
										left: None,
										right: right_left.take().unwrap()
									})),
									right: Box::new(ASTNode::Op {
										operator: '!',
										left: None,
										right: Box::new(mem::replace(right_right, ASTNode::Value('\0')))
									})
								};
								*modified = true;
							}
						_ => {}
					}
				},
				'^' => {
					*node = ASTNode::Op {
						operator: '|',
						left: Some(Box::new(ASTNode::Op {
							operator: '&',
							left: left.clone(),
							right: Box::new(ASTNode::Op {
								operator: '!',
								left: None,
								right: right.clone()
							})
						})),
						right: Box::new(ASTNode::Op {
							operator: '&',
							left: Some(Box::new(ASTNode::Op {
								operator: '!',
								left: None,
								right: left.take().unwrap()
							})),
							right: Box::new(mem::replace(right, ASTNode::Value('\0')))
						})
					};
					*modified = true;
				},
				_ => { }
			}

			let ASTNode::Op { left, right, .. } = node else { return; };
			if let Some(l) = left.as_mut() {
				tree_to_almost_nnf(l, modified);
			}
			tree_to_almost_nnf(right, modified);
		},
		_ => { }
	}
}

#[allow(non_snake_case)]
pub fn negation_normal_form(formula: &str) -> String
{
	if formula.contains('1') || formula.contains('0')
	{
		println!("Formula contains constants (0 or 1), cannot convert to NNF");
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

	tree_to_string(&tree)
}