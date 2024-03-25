// Module imports for all the opcodes
pub mod opcodes;
// Module for input processor
pub mod  processor;
// Module for execution of the code on the vm
pub mod stack;

use std::env;

use processor::input::parse_input;

use stack::executor::execute_code;


fn main() {
    println!("======================================================\nBTC VM Simulator\n======================================================");

    let args: Vec<String> = env::args().collect();

    let res = parse_input(args[1].clone()).unwrap();
    execute_code(res.clone());
}