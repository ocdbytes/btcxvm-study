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
use crate::opcodes::crypto_ops::check_sig::op_checksig;
use crate::opcodes::crypto_ops::op_check_multi_sig::op_checkmultisig;
use crate::opcodes::crypto_ops::ripe_md_160::{hash_160, ripe_md_160};
use crate::opcodes::crypto_ops::sha_1::sha_1;
use crate::opcodes::crypto_ops::sha_256::{hash_256, sha_256};
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
use color_eyre::Result;
use secp256k1::{All, Secp256k1};

/// Executes a sequence of Bitcoin script operations, maintaining both a main stack and an alternative stack.
/// Handles control flow, cryptographic operations, and basic stack manipulation according to the Bitcoin protocol.
///
/// The function processes script operations sequentially while respecting conditional execution (IF/ELSE/ENDIF)
/// and maintaining proper state for signature verification through OP_CODESEPARATOR.
///
/// Returns both the main stack and alternative stack after execution completes.
pub fn execute_code(seq: Vec<String>, secp: Secp256k1<All>) -> Result<(Stack, Stack)> {
    let mut main_stack = Stack::new();
    let mut alt_stack = Stack::new();
    let mut ops_array: Vec<String> = vec![];
    let mut control_flow = ControlFlow::new();

    // Tracks the position of the most recently executed OP_CODESEPARATOR
    // This affects which portion of the script is included in signature verification
    let mut last_code_separator_index = 0;

    for (index, code) in seq.iter().enumerate() {
        log::debug!("Processing code : {:?}", code.clone());

        // Only execute operations if we're not in a skipped branch of an IF/ELSE block
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
                // CRYPTO OPS
                // ============================================
                "OP_CHECKSIG" => {
                    if seq[last_code_separator_index..].contains(&"OP_CODESEPARATOR".to_string()) {
                        op_checksig(
                            &mut main_stack,
                            &seq[last_code_separator_index + 1..],
                            &secp,
                        )?
                    } else {
                        op_checksig(&mut main_stack, &seq, &secp)?
                    }
                }
                "OP_CHECKSIGVERIFY" => {
                    if seq[last_code_separator_index..].contains(&"OP_CODESEPARATOR".to_string()) {
                        op_checksig(
                            &mut main_stack,
                            &seq[last_code_separator_index + 1..],
                            &secp,
                        )?
                    } else {
                        op_checksig(&mut main_stack, &seq, &secp)?
                    }
                    verify(&mut main_stack)?
                }
                "OP_RIPEMD160" => ripe_md_160(&mut main_stack)?,
                "OP_SHA1" => sha_1(&mut main_stack)?,
                "OP_SHA256" => sha_256(&mut main_stack)?,
                "OP_HASH160" => hash_160(&mut main_stack)?,
                "OP_HASH256" => hash_256(&mut main_stack)?,
                "OP_CHECKMULTISIG" => {
                    if seq[last_code_separator_index..].contains(&"OP_CODESEPARATOR".to_string()) {
                        op_checkmultisig(
                            &mut main_stack,
                            &seq[last_code_separator_index + 1..],
                            &secp,
                        )?
                    } else {
                        op_checkmultisig(&mut main_stack, &seq, &secp)?
                    }
                }
                "OP_CHECKMULTISIGVERIFY" => {
                    if seq[last_code_separator_index..].contains(&"OP_CODESEPARATOR".to_string()) {
                        op_checkmultisig(
                            &mut main_stack,
                            &seq[last_code_separator_index + 1..],
                            &secp,
                        )?
                    } else {
                        op_checkmultisig(&mut main_stack, &seq, &secp)?
                    }
                    verify(&mut main_stack)?
                }

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
                "OP_CODESEPARATOR" => {
                    // When we encounter a code separator, update the index to the current position
                    // This means subsequent signature verifications will only consider script
                    // operations that come after this point
                    last_code_separator_index = index;
                }

                _ => {
                    // Handle non-opcode values (typically numbers or public keys)
                    if code.starts_with("OP_") {
                        Err(OpCodeErrors::UnknownOpcode)?
                    }
                    new_num(&mut main_stack, code.clone())?;
                    ops_array.push(code.clone());
                }
            }
        } else if code == "OP_IF" || code == "OP_ELSE" || code == "OP_ENDIF" {
            // Always process control flow operations, even in skipped branches
            // This maintains proper nesting of conditional blocks
            match code.as_str() {
                "OP_IF" => control_flow.op_if(&mut main_stack)?,
                "OP_ELSE" => control_flow.op_else()?,
                "OP_ENDIF" => control_flow.op_endif()?,
                _ => unreachable!(),
            }
        }

        // Keep track of all operations for debugging and analysis
        ops_array.push(code.clone());

        log::debug!("STACK : {:?}", &main_stack.elements);
        log::debug!("ALT STACK : {:?}", &alt_stack.elements);
    }

    // Print final state for debugging
    println!("\n======================================================\nSTACK (final) :");
    print_in_box(&mut main_stack.elements);
    println!("\nALT STACK (final) :",);
    print_in_box(&mut alt_stack.elements);
    println!("\nOPERATIONS (final) : \n{:?}", ops_array);

    Ok((main_stack, alt_stack))
}
