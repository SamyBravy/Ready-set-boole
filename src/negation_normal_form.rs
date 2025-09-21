use crate::boolean_evaluation::ASTNode;
use crate::boolean_evaluation::build_ast;
use std::mem;

fn tree_to_string(node: &ASTNode) -> String
{
	match node {
		ASTNode::Value(v) => v.to_string(),
        ASTNode::Op { operator, left, right } => {
            let mut res = String::new();

            if let Some(l) = left.as_ref() {
                res.push_str(&tree_to_string(l));
            }
            res.push_str(&tree_to_string(right));
            res.push_str(&operator.to_string());

            res
        }
	}
}

fn tree_to_nnf(node: &mut ASTNode)
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
					}
				},
				'!' => {
					match &mut **right {
						ASTNode::Op { operator: '!', right: right_right, .. } => {
							*node = mem::replace(right_right, ASTNode::Value('\0'));
							tree_to_nnf(node);
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
								}
							}
						_ => {}
					}
				},
				_ => { }
			}

			let ASTNode::Op { left, right, .. } = node else { return; };
			if let Some(l) = left.as_mut() {
				tree_to_nnf(l);
			}
			tree_to_nnf(right);
		},
		_ => { }
	}
}

#[allow(non_snake_case)]
pub fn negation_normal_form(formula: &str) -> String
{
	let mut tree = match build_ast(formula) {
		Some(ast) => ast,
		None => {
			println!("Error in formula");
			return String::new();
		}
	};

	tree_to_nnf(&mut tree);

	tree_to_string(&tree)
}