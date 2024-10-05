use crate::errors::OpCodeErrors;
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
use crate::opcodes::control_flow::ControlFlow;
use crate::opcodes::new_num::new_num;
use crate::opcodes::op_equal::op_equal;
use crate::opcodes::op_reserved::op_reserved;
use crate::opcodes::op_return::op_return;
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
use crate::stack::Stack;
use crate::utils::print_in_box;

pub fn execute_code(seq: Vec<String>) -> color_eyre::Result<(Stack, Stack)> {
    let mut main_stack = Stack::new();
    let mut alt_stack = Stack::new();
    let mut ops_array: Vec<String> = vec![];
    let mut control_flow = ControlFlow::new();

    for code in seq {
        log::debug!("Processing code : {:?}", code.clone());

        if control_flow.should_execute() {
            match code.as_str() {
                // ============================================
                // ARITHMETICS
                // ============================================
                "OP_ADD" => add(&mut main_stack)?,
                "OP_1ADD" => add_1(&mut main_stack)?,
                "OP_SUB" => sub(&mut main_stack)?,
                "OP_1SUB" => sub_1(&mut main_stack)?,
                "OP_NEGATE" => negate(&mut main_stack)?,
                "OP_ABS" => abs(&mut main_stack)?,
                "OP_NOT" => not(&mut main_stack)?,
                "OP_0NOTEQUAL" => zero_not_equal(&mut main_stack)?,
                "OP_BOOLAND" => bool_and(&mut main_stack)?,
                "OP_BOOLOR" => bool_or(&mut main_stack)?,
                "OP_NUMEQUAL" => num_equal(&mut main_stack)?,
                "OP_NUMEQUALVERIFY" => {
                    num_equal(&mut main_stack)?;
                    verify(&mut main_stack)?
                }
                "OP_NUMNOTEQUAL" => num_not_equal(&mut main_stack)?,
                "OP_LESSTHAN" => less_than(&mut main_stack)?,
                "OP_GREATERTHAN" => greater_than(&mut main_stack)?,
                "OP_LESSTHANOREQUAL" => less_than_or_equal(&mut main_stack)?,
                "OP_GREATERTHANOREQUAL" => greater_than_or_equal(&mut main_stack)?,
                "OP_MIN" => min(&mut main_stack)?,
                "OP_MAX" => max(&mut main_stack)?,
                "OP_WITHIN" => within(&mut main_stack)?,

                // ============================================
                // STACK OPS
                // ============================================
                "OP_DEPTH" => depth(&mut main_stack)?,
                "OP_DROP" => op_drop(&mut main_stack, 1)?,
                "OP_2DROP" => op_drop(&mut main_stack, 2)?,
                "OP_DUP" => dup(&mut main_stack, 1)?,
                "OP_2DUP" => dup(&mut main_stack, 2)?,
                "OP_3DUP" => dup(&mut main_stack, 3)?,
                "OP_NIP" => nip(&mut main_stack)?,
                "OP_OVER" => over(&mut main_stack)?,
                "OP_2OVER" => over_2(&mut main_stack)?,
                "OP_PICK" => pick(&mut main_stack)?,
                "OP_ROLL" => roll(&mut main_stack)?,
                "OP_ROT" => rot(&mut main_stack)?,
                "OP_2ROT" => rot_2(&mut main_stack)?,
                "OP_SWAP" => swap(&mut main_stack)?,
                "OP_2SWAP" => swap_2(&mut main_stack)?,
                "OP_TUCK" => tuck(&mut main_stack)?,
                "OP_SIZE" => size(&mut main_stack)?,
                "OP_IFDUP" => if_dup(&mut main_stack)?,

                "OP_TOALTSTACK" => to_alt_stack(&mut main_stack, &mut alt_stack)?,
                "OP_FROMALTSTACK" => from_alt_stack(&mut main_stack, &mut alt_stack)?,

                // ============================================
                // CONTROL FLOW
                // ============================================
                "OP_IF" => control_flow.op_if(&mut main_stack)?,
                "OP_ELSE" => control_flow.op_else()?,
                "OP_ENDIF" => control_flow.op_endif()?,

                // ============================================
                // OTHER OPS
                // ============================================
                "OP_TRUE" => op_true(&mut main_stack),
                "OP_FALSE" => op_false(&mut main_stack),
                "OP_VERIFY" => verify(&mut main_stack)?,
                "OP_RETURN" => op_return()?,
                "OP_EQUAL" => op_equal(&mut main_stack)?,
                "OP_EQUALVERIFY" => {
                    op_equal(&mut main_stack)?;
                    verify(&mut main_stack)?
                }
                "OP_RESERVED" => op_reserved()?,

                _ => {
                    if code.starts_with("OP_") {
                        Err(OpCodeErrors::UnknownOpcode)?
                    }
                    new_num(&mut main_stack, code.clone())?;
                    ops_array.push(code.clone());
                }
            }
        } else if code == "OP_IF" || code == "OP_ELSE" || code == "OP_ENDIF" {
            // Always process these opcodes to maintain proper nesting
            match code.as_str() {
                "OP_IF" => control_flow.op_if(&mut main_stack)?,
                "OP_ELSE" => control_flow.op_else()?,
                "OP_ENDIF" => control_flow.op_endif()?,
                _ => unreachable!(),
            }
        }

        ops_array.push(code.clone());

        log::debug!("STACK : {:?}", &main_stack.elements);
        log::debug!("ALT STACK : {:?}", &alt_stack.elements);
    }

    println!("\n======================================================\nSTACK (final) :");
    print_in_box(&mut main_stack.elements);
    println!("\nALT STACK (final) :",);
    print_in_box(&mut alt_stack.elements);
    println!("\nOPERATIONS (final) : \n{:?}", ops_array);

    Ok((main_stack, alt_stack))
}
