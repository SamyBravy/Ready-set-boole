use crate::boolean_evaluation::eval_formula;
use std::collections::HashMap;

fn is_operand(c: char) -> bool {
    !c.is_alphabetic()
}

fn unique_ids(formula: &str) -> Vec<char> {
    let mut ids: Vec<char> = Vec::new();
    for c in formula.chars() {
        if !is_operand(c) && !ids.contains(&c) {
            ids.push(c);
        }
    }
    ids.sort();
    ids
}

fn generate_truth_row(ids: &Vec<char>, row: usize) -> HashMap<char, bool> {
    let bin = format!("{:0width$b}", row, width = ids.len());
    let mut truth_row = HashMap::new();
    for (i, c) in bin.chars().enumerate() {
        truth_row.insert(ids[i], c == '1');
    }
    truth_row
}

fn evaluate(formula: &str, ids: &Vec<char>, truth_row: &HashMap<char, bool>) -> bool {
    let rpn: String = formula.chars().map(|c| {
        if !is_operand(c) {
            let index = ids.iter().position(|&x| x == c).unwrap();
            if *truth_row.get(&ids[index]).unwrap() { '1' } else { '0' }
        } else {
            c
        }
    }).collect();
    eval_formula(&rpn)
}

fn print_header(ids: &Vec<char>) {
    for &id in ids {
        print!("| {} ", id.to_ascii_uppercase());
    }
    print!("| = |");
    println!();
    for _ in 0..ids.len() {
        print!("|---");
    }
    print!("|---|");
    println!();
}

fn print_row(ids: &Vec<char>, row: &HashMap<char, bool>) {
    for &id in ids {
        let val = row.get(&id).unwrap();
        print!("| {} ", if *val { 1 } else { 0 });
    }
}

pub fn print_truth_table(formula: &str) {
    let ids = unique_ids(formula);
    print_header(&ids);
    for i in 0..(1 << ids.len()) {
        let truth_row = generate_truth_row(&ids, i);
        print_row(&ids, &truth_row);
        let result = evaluate(formula, &ids, &truth_row);
        println!("| {} |", if result { 1 } else { 0 });
    }
}
