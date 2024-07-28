use super::stack::Stack;
use crate::opcodes::abs::abs;
use crate::opcodes::add1::add_1;
use crate::opcodes::negate::negate;
use crate::opcodes::not::not;
use crate::opcodes::op_false::op_false;
use crate::opcodes::op_true::op_true;
use crate::opcodes::sub1::sub_1;
use crate::opcodes::zero_not_equal::zero_not_equal;
use crate::opcodes::{add::add, new_num::new_num, sub::sub};
use crate::opcodes::bool_and::bool_and;
use crate::opcodes::bool_or::bool_or;
use crate::opcodes::num_equal::num_equal;
use crate::opcodes::num_not_equal::num_not_equal;
use crate::opcodes::verify::verify;
use crate::stack::executor_utils::{is_numeric_string, is_op_range};
use crate::utils::print_in_box;

pub fn execute_code(seq: Vec<String>) -> color_eyre::Result<()> {
    let mut main_stack = Stack::new();
    let mut alt_stack = Stack::new();
    let mut ops_array: Vec<String> = vec![];

    for code in seq {
        println!("STACK : {:?}", &main_stack.elements);
        println!("ALT STACK : {:?}", &alt_stack.elements);
        println!(">>>> Processing code : {:?}", code.clone());

        // OP_ADD
        if code == "OP_ADD" {
            add(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_1ADD
        if code == "OP_1ADD" {
            add_1(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_SUB
        if code == "OP_SUB" {
            sub(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_1SUB
        if code == "OP_1SUB" {
            sub_1(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // For numeric input ranging in `i32`
        if is_numeric_string(&code) {
            // for direct opcode data addition such inputs as :
            // 1, 2, 3 ... 16
            if is_op_range(&code) {
                new_num(&mut main_stack, code.clone())?;
                ops_array.push(format!("OP_{}", code));
            } else {
                // for adding a number in the stack with range in i32
                new_num(&mut main_stack, code.clone())?;
                ops_array.push(format!("{}", code));
            }
        }

        // OP_TRUE
        if code == "OP_TRUE" {
            op_true(&mut main_stack);
            ops_array.push(code.clone());
        }

        // OP_FALSE
        if code == "OP_FALSE" {
            op_false(&mut main_stack);
            ops_array.push(code.clone());
        }

        // OP_NEGATE
        if code == "OP_NEGATE" {
            negate(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_ABS
        if code == "OP_ABS" {
            abs(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_NOT
        if code == "OP_NOT" {
            not(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_0NOTEQUAL
        if code == "OP_0NOTEQUAL" {
            zero_not_equal(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_BOOLAND
        if code == "OP_BOOLAND" {
            bool_and(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_BOOLOR
        if code == "OP_BOOLOR" {
            bool_or(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_NUMEQUAL
        if code == "OP_NUMEQUAL" {
            num_equal(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_NUMNOTEQUAL
        if code == "OP_NUMNOTEQUAL" {
            num_not_equal(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        //  OP_NUMEQUALVERIFY
        if code == "OP_NUMEQUALVERIFY" {
            num_equal(&mut main_stack)?;
            verify(&mut main_stack)?;
            ops_array.push(code.clone());
        }
    }

    println!("\n======================================================\nSTACK (final) :");
    print_in_box(&mut main_stack.elements);
    println!("\nALT STACK (final)",);
    print_in_box(&mut alt_stack.elements);
    println!("\nOPERATIONS (final) : {:?}", ops_array);

    Ok(())
}
