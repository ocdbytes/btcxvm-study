use crate::errors::OpCodeErrors;
use crate::stack::Stack;

pub fn op_equal(stack: &mut Stack) -> Result<(), OpCodeErrors> {
    if stack.length < 2 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is invalid".to_string(),
        ));
    }

    // we can use unwrap() as check is being done above
    let b = stack.pop_from_top().unwrap();
    let a = stack.pop_from_top().unwrap();

    if a == b {
        stack.push_to_top("1".to_string());
    } else {
        stack.push_to_top("0".to_string());
    }

    Ok(())
}
