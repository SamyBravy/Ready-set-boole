#[cfg(test)]
mod tests {
    use crate::adder;
    use crate::boolean_evaluation;
    use crate::conjunctive_normal_form;
    use crate::gray_code;
    use crate::multiplier;
    use crate::negation_normal_form;
    use crate::powerset;
    use crate::sat;
    use crate::set_evaluation;
    use crate::space_filling;
    use crate::truth_table;

    #[test]
    fn test_adder() {
        assert_eq!(adder::adder(5, 7), 12);
        assert_eq!(adder::adder(15, 27), 42);
        assert_eq!(adder::adder(0, 0), 0);
        assert_eq!(adder::adder(1, 1), 2);
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier::multiplier(15, 27), 405);
        assert_eq!(multiplier::multiplier(30, 40), 1200);
        assert_eq!(multiplier::multiplier(0, 5), 0);
        assert_eq!(multiplier::multiplier(1, 1), 1);
    }

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code::gray_code(10), 15);
        assert_eq!(gray_code::gray_code(25), 21); // corrected from main.rs
        assert_eq!(gray_code::gray_code(0), 0);
        assert_eq!(gray_code::gray_code(1), 1);
    }

    #[test]
    fn test_powerset() {
        assert_eq!(
            powerset::powerset(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
        assert_eq!(powerset::powerset(vec![]), vec![vec![]]);
        assert_eq!(powerset::powerset(vec![5]), vec![vec![], vec![5]]);
    }

    #[test]
    fn test_space_filling_map() {
        assert_eq!(space_filling::map(5, 7), 59.0);
        assert_eq!(space_filling::map(0, 0), 0.0);
    }

    #[test]
    fn test_boolean_evaluation_build_ast() {
        assert!(boolean_evaluation::build_ast("10&").is_some());
        assert!(boolean_evaluation::build_ast("AB&").is_some());
        assert!(boolean_evaluation::build_ast("invalid").is_none());
    }

    #[test]
    fn test_boolean_evaluation_eval_formula() {
        assert_eq!(boolean_evaluation::eval_formula("10&"), false);
        assert_eq!(boolean_evaluation::eval_formula("10|"), true);
        assert_eq!(boolean_evaluation::eval_formula("11>"), true);
        assert_eq!(boolean_evaluation::eval_formula("10="), false);
    }

    #[test]
    fn test_truth_table_substitute_vars() {
        use std::collections::BTreeMap;
        let mut dict = BTreeMap::new();
        dict.insert('A', true);
        dict.insert('B', false);
        assert_eq!(truth_table::substitute_vars("AB&", &dict), "10&");
    }

    #[test]
    fn test_truth_table_create_dict() {
        let dict = truth_table::create_dict("AB&C");
        assert!(dict.contains_key(&'A'));
        assert!(dict.contains_key(&'B'));
        assert!(dict.contains_key(&'C'));
    }

    #[test]
    fn test_truth_table_update_dict() {
        use std::collections::BTreeMap;
        let mut dict = BTreeMap::new();
        dict.insert('A', false);
        dict.insert('B', false);
        truth_table::update_dict(&mut dict, 1);
        assert_eq!(*dict.get(&'B').unwrap(), true);
    }

    #[test]
    fn test_truth_table_print_truth_table() {
        // Just call to ensure no panic
        truth_table::print_truth_table("AB&C");
    }

    #[test]
    fn test_sat() {
        assert_eq!(sat::sat("AB|"), true);
        assert_eq!(sat::sat("AB&"), true);
        assert_eq!(sat::sat("AA!&"), false);
    }

    #[test]
    fn test_negation_normal_form() {
        assert_eq!(negation_normal_form::negation_normal_form("AB&!"), "B!A!|");
    }

    #[test]
    fn test_conjunctive_normal_form() {
        assert_eq!(
            conjunctive_normal_form::conjunctive_normal_form("AB|!"),
            "B!A!&"
        );
    }

    #[test]
    fn test_set_evaluation_eval_set() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        assert_eq!(set_evaluation::eval_set("AB&", sets), vec![0]);
    }

    // For helper functions like tree_to_string, tree_to_almost_nnf, etc., perhaps skip or test indirectly
}
