use crate::boolean_evaluation::{build_ast, ASTNode};
use crate::negation_normal_form::tree_to_almost_nnf;
use std::ops::{BitAnd, BitOr, Not};
use std::process::exit;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct MySet(Vec<i32>);

static UNIVERSE: Lazy<Mutex<MySet>> = Lazy::new(|| Mutex::new(MySet(vec![])));

impl BitAnd for MySet {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        MySet(
            self.0
                .iter()
                .filter_map(|n| if other.0.contains(n) { Some(*n) } else { None })
                .collect::<Vec<i32>>(),
        )
    }
}

impl BitOr for MySet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        MySet(
            [
                self.0.clone(),
                other
                    .0
                    .iter()
                    .filter_map(|n| if !self.0.contains(n) { Some(*n) } else { None })
                    .collect(),
            ]
            .concat(),
        )
    }
}

impl Not for MySet {
    type Output = Self;

    fn not(self) -> Self::Output {
		let universe = UNIVERSE.lock().unwrap();
        MySet(
            universe.0
			.iter()
			.filter_map(|n| if !self.0.contains(n) { Some(*n) } else { None })
			.collect()
        )
    }
}

fn build_universe(sets: &Vec<Vec<i32>>) -> MySet {
    let mut all = Vec::new();

    for s in sets {
        for x in s {
            if !all.contains(x) {
                all.push(*x);
            }
        }
    }

    MySet(all)
}

fn vec_tree(node: ASTNode<char>, sets: &Vec<Vec<i32>>) -> ASTNode<MySet> {
    match node {
        ASTNode::Value(c) => {
            let idx = (c as u8 - b'A') as usize;
			if idx >= sets.len() {
				println!("Index out of bounds for sets");
				exit(1);
			}
            ASTNode::Value(MySet(sets[idx].clone()))
        }
        ASTNode::Op {
            operator,
            left,
            right,
        } => {
            let new_left = left.map(|l| Box::new(vec_tree(*l, sets)));
            let new_right = Box::new(vec_tree(*right, sets));
            ASTNode::Op {
                operator,
                left: new_left,
                right: new_right,
            }
        }
    }
}

fn eval_node_vecs(node: ASTNode<MySet>) -> MySet {
    match node {
        ASTNode::Value(v) => v,
        ASTNode::Op {
            operator,
            left,
            right,
        } => match operator {
            '&' => eval_node_vecs(*left.unwrap()) & eval_node_vecs(*right),
            '|' => eval_node_vecs(*left.unwrap()) | eval_node_vecs(*right),
            '!' => !eval_node_vecs(*right),
            _ => {
                println!("Invalid operator in vec AST");
                MySet(vec![])
            }
        },
    }
}

#[allow(non_snake_case)]
pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    if formula.contains('1') || formula.contains('0') {
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
    while modified {
        modified = false;
        tree_to_almost_nnf(&mut tree, &mut modified);
    }

    let tree = vec_tree(tree, &sets);

	{
        let mut u = UNIVERSE.lock().unwrap();
        *u = build_universe(&sets);
    }
    eval_node_vecs(tree).0
}
