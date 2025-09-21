//! Direct RPN -> Negation Normal Form conversion using an AST.
//!
//! Supported operators (single char tokens):
//!   ! (NOT), & (AND), | (OR), > (IMPLIES), = (EQUIVALENCE), ^ (XOR)
//! Operands: A-Z a-z 0-9 (single characters).
//! Input is postfix (RPN). Output NNF can be requested as infix or RPN.

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Var(char),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    // Only during initial parse (will be desugared):
    Imp(Box<Node>, Box<Node>),
    Iff(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
}

// -------- Parsing (RPN -> AST) --------
fn parse_rpn(rpn: &str) -> Result<Node, String> {
    let mut stack: Vec<Node> = Vec::new();
    for ch in rpn.chars().filter(|c| !c.is_whitespace()) {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' => stack.push(Node::Var(ch)),
            '!' => {
                let a = stack.pop().ok_or("underflow for !")?;
                stack.push(Node::Not(Box::new(a)));
            }
            '&' => bin(&mut stack, Node::And)?,
            '|' => bin(&mut stack, Node::Or)?,
            '>' => bin(&mut stack, Node::Imp)?,
            '=' => bin(&mut stack, Node::Iff)?,
            '^' => bin(&mut stack, Node::Xor)?,
            other => return Err(format!("unexpected token {other}")),
        }
    }
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("extra operands or empty input".into())
    }
}

fn bin<F>(st: &mut Vec<Node>, f: F) -> Result<(), String>
where
    F: Fn(Box<Node>, Box<Node>) -> Node,
{
    let r = st.pop().ok_or("underflow (rhs)")?;
    let l = st.pop().ok_or("underflow (lhs)")?;
    st.push(f(Box::new(l), Box::new(r)));
    Ok(())
}

// -------- Desugaring (> = ^) --------
fn desugar(n: Node) -> Node {
    match n {
        Node::Imp(a, b) => Node::Or(
            Box::new(Node::Not(Box::new(desugar(*a)))),
            Box::new(desugar(*b)),
        ),
        Node::Iff(a, b) => {
            // a <-> b  == (a & b) | (!a & !b)
            let a_ = desugar(*a);
            let b_ = desugar(*b);
            Node::Or(
                Box::new(Node::And(Box::new(a_.clone()), Box::new(b_.clone()))),
                Box::new(Node::And(
                    Box::new(Node::Not(Box::new(a_))),
                    Box::new(Node::Not(Box::new(b_))),
                )),
            )
        }
        Node::Xor(a, b) => {
            // a XOR b == (a & !b) | (!a & b)
            let a_ = desugar(*a);
            let b_ = desugar(*b);
            Node::Or(
                Box::new(Node::And(
                    Box::new(a_.clone()),
                    Box::new(Node::Not(Box::new(b_.clone()))),
                )),
                Box::new(Node::And(Box::new(Node::Not(Box::new(a_))), Box::new(b_))),
            )
        }
        Node::And(a, b) => Node::And(Box::new(desugar(*a)), Box::new(desugar(*b))),
        Node::Or(a, b) => Node::Or(Box::new(desugar(*a)), Box::new(desugar(*b))),
        Node::Not(a) => Node::Not(Box::new(desugar(*a))),
        Node::Var(c) => Node::Var(c),
    }
}

// -------- Push negations inward (De Morgan + !!A) --------
fn push_neg(n: Node) -> Node {
    match n {
        Node::Not(inner) => match *inner {
            Node::Not(x) => push_neg(*x),
            Node::And(a, b) => Node::Or(
                Box::new(push_neg(Node::Not(a))),
                Box::new(push_neg(Node::Not(b))),
            ),
            Node::Or(a, b) => Node::And(
                Box::new(push_neg(Node::Not(a))),
                Box::new(push_neg(Node::Not(b))),
            ),
            Node::Var(c) => Node::Not(Box::new(Node::Var(c))),
            other => Node::Not(Box::new(push_neg(other))),
        },
        Node::And(a, b) => Node::And(Box::new(push_neg(*a)), Box::new(push_neg(*b))),
        Node::Or(a, b) => Node::Or(Box::new(push_neg(*a)), Box::new(push_neg(*b))),
        Node::Var(c) => Node::Var(c),
        // Should not appear post-desugar, but handle defensively.
        Node::Imp(_, _) | Node::Iff(_, _) | Node::Xor(_, _) => push_neg(desugar(n)),
    }
}

// -------- RPN Printing & Public API --------
fn to_rpn(n: &Node, out: &mut String) {
    match n {
        Node::Var(c) => out.push(*c),
        Node::Not(a) => { to_rpn(a, out); out.push('!'); }
        Node::And(a, b) => { to_rpn(a, out); to_rpn(b, out); out.push('&'); }
        Node::Or(a, b)  => { to_rpn(a, out); to_rpn(b, out); out.push('|'); }
        // The following variants should be eliminated by desugar before printing:
        Node::Imp(a,b) => { to_rpn(a,out); to_rpn(b,out); out.push('>'); }
        Node::Iff(a,b) => { to_rpn(a,out); to_rpn(b,out); out.push('='); }
        Node::Xor(a,b) => { to_rpn(a,out); to_rpn(b,out); out.push('^'); }
    }
}

pub fn rpn_to_nnf_rpn(rpn: &str) -> Result<String, String> {
    let ast = parse_rpn(rpn)?;
    let des = desugar(ast);
    let nnf = push_neg(des);
    let mut s = String::new();
    to_rpn(&nnf, &mut s);
    Ok(s)
}

pub fn negation_normal_form(formula: &str) -> String {
    rpn_to_nnf_rpn(formula).unwrap_or_default()
}

// -------- Tests --------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implication() { assert_eq!(rpn_to_nnf_rpn("AB>").unwrap(), "A!B|"); }
    fn equivalence() { assert_eq!(rpn_to_nnf_rpn("AB=").unwrap(), "AB&A!B!&|"); }
    fn xor() { assert_eq!(rpn_to_nnf_rpn("AB^").unwrap(), "AB!&A!B&|"); }
    fn de_morgan_and() { assert_eq!(rpn_to_nnf_rpn("AB&!").unwrap(), "A!B!|"); }
    fn double_neg() { assert_eq!(rpn_to_nnf_rpn("A!!").unwrap(), "A"); }
    fn complex() { assert_eq!(rpn_to_nnf_rpn("AB|C&!").unwrap(), "A!B!&C!|"); }
}
//  println!("{}", negation_normal_form::negation_normal_form("AB|!"));
//     // A!B!&
//     println!("{}", negation_normal_form::negation_normal_form("AB>"));
//     // A!B|
//     println!("{}", negation_normal_form::negation_normal_form("AB="));
//     // AB&A!B!&|
//     println!("{}", negation_normal_form::negation_normal_form("AB|C&!"));
//     // A!B!&C!|