use crate::errors::OpCodeErrors;

pub fn op_return() -> Result<(), OpCodeErrors> {
    Err(OpCodeErrors::OpReturn)
}
