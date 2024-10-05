use crate::errors::OpCodeErrors;

pub fn op_reserved() -> Result<(), OpCodeErrors> {
    Err(OpCodeErrors::OpReserved)
}
