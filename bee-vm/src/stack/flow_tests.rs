#[cfg(test)]
mod tests {
    use crate::stack::executor::execute_code;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["2", "3", "OP_ADD"],
        vec!["5"],
        vec![],
        "Basic arithmetic"
    )]
    #[case(
        vec!["1", "2", "3", "OP_DUP", "OP_ROT", "OP_SWAP"],
        vec!["1", "3", "2", "3"],
        vec![],
        "Stack operations"
    )]
    #[case(
        vec!["1", "2", "OP_TOALTSTACK", "3", "OP_FROMALTSTACK"],
        vec!["1", "3", "2"],
        vec![],
        "Alt stack operations"
    )]
    #[case(
        vec!["1", "OP_IF", "2", "OP_ENDIF"],
        vec!["2"],
        vec![],
        "Simple if execution"
    )]
    #[case(
        vec!["0", "OP_IF", "2", "OP_ELSE", "3", "OP_ENDIF"],
        vec!["3"],
        vec![],
        "If-else execution"
    )]
    #[case(
        vec!["1", "OP_IF", "2", "OP_IF", "3", "OP_ENDIF", "4", "OP_ENDIF"],
        vec!["3", "4"],
        vec![],
        "Nested if statements"
    )]
    #[case(
        vec!["1", "OP_IF", "OP_ENDIF"],
        vec![],
        vec![],
        "Empty if block"
    )]
    #[case(
        vec!["1", "OP_IF", "2", "3", "OP_ADD", "OP_ELSE", "5", "OP_ENDIF"],
        vec!["5"],
        vec![],
        "Mixing control flow and arithmetic"
    )]
    fn test_execute_code_success(
        #[case] opcodes: Vec<&str>,
        #[case] expected_main: Vec<&str>,
        #[case] expected_alt: Vec<&str>,
        #[case] test_name: &str,
    ) -> color_eyre::Result<()> {
        let opcodes = opcodes.into_iter().map(String::from).collect();
        let (main_stack, alt_stack) = execute_code(opcodes)?;

        assert_eq!(
            main_stack,
            Stack::stack_from(vec_str_to_vec_string(expected_main)),
            "Failed test: {}",
            test_name
        );
        assert_eq!(
            alt_stack,
            Stack::stack_from(vec_str_to_vec_string(expected_alt)),
            "Failed test: {}",
            test_name
        );

        Ok(())
    }

    #[rstest]
    #[case(vec!["OP_IF"], "Unbalanced if")]
    #[case(vec!["OP_ELSE"], "Unbalanced else")]
    #[case(vec!["OP_ENDIF"], "Unbalanced endif")]
    #[case(vec!["1", "OP_ELSE"], "Else without if")]
    #[case(vec!["1", "OP_ENDIF"], "Endif without if")]
    #[case(vec!["OP_IF", "1", "OP_ENDIF"], "If without condition")]
    #[case(vec!["OP_UNKNOWN"], "Unknown opcode")]
    fn test_execute_code_failure(#[case] opcodes: Vec<&str>, #[case] test_name: &str) {
        let opcodes = opcodes.into_iter().map(String::from).collect();
        let result = execute_code(opcodes);
        println!(">>> result : {:?}", result);
        assert!(result.is_err(), "Expected error for test: {}", test_name);
    }

    #[rstest]
    #[case(
        vec!["1", "OP_IF", "2", "OP_IF", "3", "OP_ELSE", "4", "OP_ENDIF", "OP_ELSE", "5", "OP_IF", "6", "OP_ENDIF", "OP_ENDIF"],
        vec!["3"],
        vec![],
        "Complex control flow"
    )]
    fn test_complex_control_flow(
        #[case] opcodes: Vec<&str>,
        #[case] expected_main: Vec<&str>,
        #[case] expected_alt: Vec<&str>,
        #[case] test_name: &str,
    ) -> color_eyre::Result<()> {
        let opcodes = opcodes.into_iter().map(String::from).collect();
        let (main_stack, alt_stack) = execute_code(opcodes)?;

        assert_eq!(
            main_stack,
            Stack::stack_from(vec_str_to_vec_string(expected_main)),
            "Failed test: {}",
            test_name
        );
        assert_eq!(
            alt_stack,
            Stack::stack_from(vec_str_to_vec_string(expected_alt)),
            "Failed test: {}",
            test_name
        );

        Ok(())
    }

    fn vec_str_to_vec_string(str_vec: Vec<&str>) -> Vec<String> {
        str_vec.iter().map(|&s| s.to_string()).collect()
    }
}
