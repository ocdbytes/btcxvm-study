use crate::errors::OpCodeErrors;
use crate::opcodes::utils::{check_string_type, StringType};
use crate::stack::Stack;

/// **OP_SIZE**
///
/// Pushes the string length of the top element of the stack_ops (without popping it).
///
/// [ OP_SIZE "fruit" ]
/// => [ 5 "fruit" ]
pub fn size(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length == 0 {
        return Err(OpCodeErrors::MissingValue("Stack is empty.".to_string()));
    }

    let top_element = vm_state.read_ele_from_top(0).unwrap();
    let string_type = check_string_type(top_element);

    let size = match string_type {
        StringType::STRING(val) => val.len() as i32,
        StringType::DECIMAL(val) => {
            let bits = val.checked_ilog2().unwrap_or(0) + 1;
            let bytes = (bits + 7) / 8;
            bytes as i32
        }
        StringType::HEX(val) => {
            let val = val.replace("0x", "");
            match hex::decode(&val) {
                Ok(decoded_val) => decoded_val.len() as i32,
                Err(_) => {
                    return Err(OpCodeErrors::InvalidHex(
                        "Failed to decode hex string.".to_string(),
                    ))
                }
            }
        }
    };

    vm_state.push_to_top(size.to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_size {
    use crate::opcodes::stack_ops::size::size;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".to_string()], vec!["1".to_string(), "1".to_string()])]
    #[case(vec!["1234".to_string()], vec!["1234".to_string(), "2".to_string()])]
    #[case(vec!["0x123456".to_string()], vec!["0x123456".to_string(), "3".to_string()])]
    #[case(vec!["fruit".to_string()], vec!["fruit".to_string(), "5".to_string()])]
    fn test_size(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = size(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
