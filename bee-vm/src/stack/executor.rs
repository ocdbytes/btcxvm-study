use crate::opcodes::arithmetic_ops::abs::abs;
use crate::opcodes::arithmetic_ops::add::add;
use crate::opcodes::arithmetic_ops::add1::add_1;
use crate::opcodes::arithmetic_ops::bool_and::bool_and;
use crate::opcodes::arithmetic_ops::bool_or::bool_or;
use crate::opcodes::arithmetic_ops::greater_than::greater_than;
use crate::opcodes::arithmetic_ops::greater_than_or_equal::greater_than_or_equal;
use crate::opcodes::arithmetic_ops::less_than::less_than;
use crate::opcodes::arithmetic_ops::less_than_or_equal::less_than_or_equal;
use crate::opcodes::arithmetic_ops::max::max;
use crate::opcodes::arithmetic_ops::min::min;
use crate::opcodes::arithmetic_ops::negate::negate;
use crate::opcodes::arithmetic_ops::not::not;
use crate::opcodes::arithmetic_ops::num_equal::num_equal;
use crate::opcodes::arithmetic_ops::num_not_equal::num_not_equal;
use crate::opcodes::arithmetic_ops::op_false::op_false;
use crate::opcodes::arithmetic_ops::op_true::op_true;
use crate::opcodes::arithmetic_ops::sub::sub;
use crate::opcodes::arithmetic_ops::sub1::sub_1;
use crate::opcodes::arithmetic_ops::verify::verify;
use crate::opcodes::arithmetic_ops::within::within;
use crate::opcodes::new_num::new_num;
use crate::opcodes::stack_ops::depth::depth;
use crate::opcodes::stack_ops::drop::op_drop;
use crate::opcodes::stack_ops::dup::dup;
use crate::opcodes::stack_ops::from_alt_stack::from_alt_stack;
use crate::opcodes::stack_ops::if_dup::if_dup;
use crate::opcodes::stack_ops::nip::nip;
use crate::opcodes::stack_ops::over::{over, over_2};
use crate::opcodes::stack_ops::pick::pick;
use crate::opcodes::stack_ops::roll::roll;
use crate::opcodes::stack_ops::rot::{rot, rot_2};
use crate::opcodes::stack_ops::size::size;
use crate::opcodes::stack_ops::swap::{swap, swap_2};
use crate::opcodes::stack_ops::to_alt_stack::to_alt_stack;
use crate::opcodes::stack_ops::tuck::tuck;
use crate::opcodes::zero_not_equal::zero_not_equal;
use crate::stack::executor_utils::{is_numeric_string, is_op_range};
use crate::stack::Stack;
use crate::utils::print_in_box;

pub fn execute_code(seq: Vec<String>) -> color_eyre::Result<()> {
    let mut main_stack = Stack::new();
    let mut alt_stack = Stack::new();
    let mut ops_array: Vec<String> = vec![];

    for code in seq {
        println!("STACK : {:?}", &main_stack.elements);
        println!("ALT STACK : {:?}", &alt_stack.elements);
        println!(">>>> Processing code : {:?}", code.clone());

        // ============================================
        // ARITHMETICS
        // ============================================

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
                // for adding a number in the stack_ops with range in i32
                new_num(&mut main_stack, code.clone())?;
                ops_array.push(code.to_string());
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

        // OP_LESSTHAN
        if code == "OP_LESSTHAN" {
            less_than(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_GREATERTHAN
        if code == "OP_GREATERTHAN" {
            greater_than(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_LESSTHANOREQUAL
        if code == "OP_LESSTHANOREQUAL" {
            less_than_or_equal(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_GREATERTHANOREQUAL
        if code == "OP_GREATERTHANOREQUAL" {
            greater_than_or_equal(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_MIN
        if code == "OP_MIN" {
            min(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_MAX
        if code == "OP_MAX" {
            max(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_WITHIN
        if code == "OP_WITHIN" {
            within(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // ============================================
        // STACK OPS
        // ============================================

        // OP_DEPTH
        if code == "OP_DEPTH" {
            depth(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_DROP
        if code == "OP_DROP" {
            op_drop(&mut main_stack, 1)?;
            ops_array.push(code.clone());
        }

        // OP_2DROP
        if code == "OP_2DROP" {
            op_drop(&mut main_stack, 2)?;
            ops_array.push(code.clone());
        }

        // OP_DUP
        if code == "OP_DUP" {
            dup(&mut main_stack, 1)?;
            ops_array.push(code.clone());
        }

        // OP_2DUP
        if code == "OP_2DUP" {
            dup(&mut main_stack, 2)?;
            ops_array.push(code.clone());
        }

        // OP_3DUP
        if code == "OP_3DUP" {
            dup(&mut main_stack, 3)?;
            ops_array.push(code.clone());
        }

        // OP_IFDUP
        if code == "OP_IFDUP" {
            if_dup(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_NIP
        if code == "OP_NIP" {
            nip(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_NIP
        if code == "OP_NIP" {
            nip(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_OVER
        if code == "OP_OVER" {
            over(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_2OVER
        if code == "OP_2OVER" {
            over_2(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_PICK
        if code == "OP_PICK" {
            pick(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_TOALTSTACK
        if code == "OP_TOALTSTACK" {
            to_alt_stack(&mut main_stack, &mut alt_stack)?;
            ops_array.push(code.clone());
        }

        // OP_FROMALTSTACK
        if code == "OP_FROMALTSTACK" {
            from_alt_stack(&mut main_stack, &mut alt_stack)?;
            ops_array.push(code.clone());
        }

        // OP_ROT
        if code == "OP_ROT" {
            rot(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_2ROT
        if code == "OP_2ROT" {
            rot_2(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_SWAP
        if code == "OP_SWAP" {
            swap(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_2SWAP
        if code == "OP_2SWAP" {
            swap_2(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_TUCK
        if code == "OP_TUCK" {
            tuck(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_SIZE
        if code == "OP_SIZE" {
            size(&mut main_stack)?;
            ops_array.push(code.clone());
        }

        // OP_ROLL
        if code == "OP_ROLL" {
            roll(&mut main_stack)?;
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
