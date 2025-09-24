use crate::boolean_evaluation::build_ast;
use crate::boolean_evaluation::eval_node;
use std::collections::BTreeMap;

pub fn substitute_vars(formula: &str, dict: &BTreeMap<char, bool>) -> String {
    let mut new_formula = formula.to_string();
    for (key, value) in dict.iter() {
        let replacement = if *value { '1' } else { '0' };
        new_formula = new_formula.replace(*key, &replacement.to_string());
    }
    new_formula
}

pub fn create_dict(formula: &str) -> BTreeMap<char, bool> {
    let mut dict: BTreeMap<char, bool> = BTreeMap::new();
    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            dict.insert(c, false);
        }
        if c == '0' || c == '1' {
            println!("Formula contains constants");
            return BTreeMap::new();
        }
    }
    dict
}

pub fn update_dict(dict: &mut BTreeMap<char, bool>, i: i32) {
    let mut j = i;
    for val in dict.values_mut().rev() {
        *val = (j & 1) == 1;
        j >>= 1;
    }
}

pub fn print_truth_table(formula: &str) {
    if build_ast(formula).is_none() {
        println!("Error in formula");
        return;
    }

    let mut dict = create_dict(formula);
    if dict.is_empty() {
        println!("No variables in formula");
        return;
    }

    for key in dict.keys() {
        print!("| {} ", key);
    }
    println!("| = |");
    println!("{}|", "|---".repeat(dict.len() + 1));

    for i in 0..(1 << dict.len()) {
        update_dict(&mut dict, i);

        let new_formula = substitute_vars(&formula, &dict);
        let tree = build_ast(&new_formula).unwrap();

        for value in dict.values() {
            print!("| {} ", if *value { '1' } else { '0' });
        }
        println!("| {} |", if eval_node(&tree).unwrap() { '1' } else { '0' });
    }
}
