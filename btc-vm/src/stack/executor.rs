use crate::opcodes::{add::add, new_num::new_num, sub::sub};

use super::stack::Stack;

pub fn execute_code(seq: Vec<String>) {
    let mut vm_state = Stack::new();

    for code in seq {
        println!("STACK : {:?}", &vm_state.elements);
        println!(">>>> Processing code : {:?}", code);

        // ADD OPERATION
        if code == "OP_ADD" {
            add(&mut vm_state)
        }

        // SUB OPERATION
        if code == "OP_SUB" {
            sub(&mut vm_state)
        }

        // for numeric input
        if is_numeric_string(&code) {
            // for direct opcode data addition
            if is_op_range(&code) {
                new_num(&mut vm_state, code)
            }
        }
    }

    println!(
        "\n--------------------STACK (final) : {:?}",
        &vm_state.elements
    );
}

fn is_numeric_string(s: &String) -> bool {
    for c in s.chars() {
        if c.is_numeric() == false {
            return false;
        }
    }

    true
}

fn is_op_range(s: &String) -> bool {
    let num = s.parse::<u32>().unwrap();
    if num > 0 && num <= 16 {
        return true;
    }

    false
}
