use crate::errors::InputParsingError;

pub fn parse_input(input_codes: String) -> Result<Vec<String>, InputParsingError> {
    let input_codes_vector: Vec<&str> = input_codes.split_ascii_whitespace().collect();

    if input_codes_vector.len() <= 2 {
        return Err(InputParsingError::InputParsingErrorAtRun);
    }

    let mut res: Vec<String> = vec![];

    for code in &input_codes_vector {
        // println!("Code : {:?}", code);
        res.push(code.to_string());
    }

    Ok(res)
}
