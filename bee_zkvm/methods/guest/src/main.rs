use risc0_zkvm::guest::env;

fn main() {
    let input: Vec<String> = env::read();

    let (main_stack, alt_stack) = bee_vm::stack::executor::execute_code(input.clone()).expect("Execution Failed !!!");

    let mut commit_vector = vec![];
    commit_vector.push((input.len() as usize).to_string());
    commit_vector.extend(input);
    commit_vector.push(main_stack.length.to_string());
    commit_vector.extend(main_stack.elements);
    commit_vector.push(alt_stack.length.to_string());
    commit_vector.extend(alt_stack.elements);

    env::commit(&commit_vector);
}
