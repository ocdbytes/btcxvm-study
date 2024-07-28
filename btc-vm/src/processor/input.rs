pub fn parse_input(input_codes: String) -> Result<Vec<String>, &'static str> {
    let input_codes_vector: Vec<&str> = input_codes.split_ascii_whitespace().collect();

    if input_codes_vector.len() <= 2 {
        return Err("Invalid Opcodes input length.\ninput sequence must have length greater than 2\nEg :\n1 2 OP_ADD");
    }

    let mut res: Vec<String> = vec![];

    for code in &input_codes_vector {
        // println!("Code : {:?}", code);
        res.push(code.to_string());
    }

    // res.reverse();

    println!("\n>>>>> OPERATIONS : {:?}\n", res);

    Ok(res)
}
