#[derive(Clone)]
pub enum ASTNode {
    Value(char),
    Op {
        operator: char,
        left: Option<Box<ASTNode>>,
        right: Box<ASTNode>,
    },
}

pub fn build_ast(formula: &str) -> Option<ASTNode>
{
	let mut stack: Vec<ASTNode> = Vec::new();

	for c in formula.chars()
	{
		if c == '0' || c == '1' || c >= 'A' && c <= 'Z'
		{
			stack.push(ASTNode::Value(c));
		}
		else
		{
			if !['!', '&', '|', '^', '>', '='].contains(&c) || stack.len() < 1 || (stack.len() < 2 && c != '!' )
			{
				return None;
			}

			let node = ASTNode::Op {
				operator: c,
				right: Box::new(stack.pop().unwrap()),
				left: if c != '!' { Some(Box::new(stack.pop().unwrap())) } else { None },
			};
			stack.push(node)
		}
	}

	if stack.len() != 1
	{
		return None;
	}

	return Some(stack.pop().unwrap());
}

pub fn eval_node(node: &ASTNode) -> Option<bool>
{
    match node {
        ASTNode::Value(c) => match c {
			'0' => Some(false),
			'1' => Some(true),
			_ => {
				println!("Invalid value in AST");
				None
			}
		},

        ASTNode::Op { operator, left, right } => {
            match operator {
                '&' => Some(eval_node(left.as_ref()?)? & eval_node(right)?),
                '|' => Some(eval_node(left.as_ref()?)? | eval_node(right)?),
                '!' => Some(!eval_node(right)?),
                '=' => Some(eval_node(left.as_ref()?)? == eval_node(right)?),
                '>' => Some(!eval_node(left.as_ref()?)? | eval_node(right)?),
                '^' => Some(eval_node(left.as_ref()?)? ^ eval_node(right)?),
                _ => {
                    println!("Invalid operator in AST");
                    None
                }
            }
        }
    }
}

pub fn eval_formula(formula: &str) -> bool
{
	let tree = build_ast(formula);

	if tree.is_none()
	{
		println!("Error in formula");
		return false;
	}

	if let Some(result) = eval_node(&tree.unwrap())
	{
		return result;
	}
	else
	{
		return false;
	}
}
