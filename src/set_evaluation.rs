use crate::boolean_evaluation::{build_ast, ASTNode};
use crate::negation_normal_form::tree_to_almost_nnf;

use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Debug, PartialEq)]
pub struct MySet(Vec<i32>);

impl BitAnd for MySet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let v = self.0.into_iter().filter(|x| rhs.0.contains(x)).collect();
        MySet(v)
    }
}

impl BitOr for MySet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut v = self.0.clone();
        for x in rhs.0 {
            if !v.contains(&x) { v.push(x); }
        }
        MySet(v)
    }
}

impl Not for MySet {
	type Output = Self;
	fn not(self) -> Self::Output {
		let mut v = vec![];
		for x in 0..=100 { // assuming universe is 0 to 100
			if !self.0.contains(&x) { v.push(x); }
		}
		MySet(v)
	}
}


fn substitute_vecs(node: &mut ASTNode, sets: &Vec<Vec<i32>>) {
    match node {
        ASTNode::Value(c) => {
            *node = ASTNode::Set(MySet(sets[(*c as u8 - b'A') as usize].clone()));
        }
        ASTNode::Op { left, right, .. } => {
            if let Some(left_node) = left.as_mut() {
                substitute_vecs(left_node, sets);
            }
            substitute_vecs(right, sets);
        }
        _ => {}
    }
}

fn eval_node_vecs(node: ASTNode) -> MySet
{
	match node {
        ASTNode::Set(v) => v,
        ASTNode::Op { operator, left, right } => {
            match operator {
                '&' => eval_node_vecs(*left.unwrap()) & eval_node_vecs(*right),
                '|' => eval_node_vecs(*left.unwrap()) | eval_node_vecs(*right),
                '!' => !eval_node_vecs(*right),
                _ => {
                    println!("Invalid operator in AST");
					MySet(vec![])
                }
            }
        },
		_ => {
			println!("Invalid node in AST");
			MySet(vec![])
		}
	}
}

#[allow(non_snake_case)]
pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>
{
	if formula.contains('1') || formula.contains('0')
	{
		println!("Formula contains constants (0 or 1), cannot convert to NNF");
		return vec![];
	}
	let mut tree = match build_ast(formula) {
		Some(ast) => ast,
		None => {
			println!("Error in formula");
			return vec![];
		}
	};

	let mut modified = true;
	while modified
	{
		modified = false;
		tree_to_almost_nnf(&mut tree, &mut modified);
	}

	substitute_vecs(&mut tree, &sets);

	eval_node_vecs(tree).0
}