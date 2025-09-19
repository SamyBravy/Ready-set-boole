
pub fn eval_formula(formula: &str) -> bool{
	let mut stack: Vec<bool> = Vec::new();
	for c in formula.chars() {
		match c {
			'0' => stack.push(false),
			'1' => stack.push(true),
		    '&' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a & b);
                }
            },
            '|' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a | b);
                }
            },
            '!' => {
                if let Some(a) = stack.pop() {
                    stack.push(!a);
                }
            },
            _ => {
				println!("Invalid character in formula");
				return false;
			}
        }
    }
	if stack.len() != 1 {
		println!("Error in formula");
		return false;
	}
	return stack.pop().unwrap_or(false);
}