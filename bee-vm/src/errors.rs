use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OpCodeErrors {
    #[error("Value missing in stack_ops while executing the opcode : {0}")]
    MissingValue(String),
    #[error("Values missing in stack_ops while executing the opcode : {0}")]
    MissingValues(String),
    #[error("Error converting the input into the range of -2^31 to 2^31 - 1 (i32)")]
    NumberNotInRange,
    #[error("Verification failed")]
    OpVerifyFailed,
    #[error("Error : {0}")]
    NIsLargerThanOrEqualToStackSize(String),
}

#[derive(Error, Debug)]
pub enum InputParsingError {
    #[error("Not able to parse the inputs provided to the vm.")]
    InputParsingErrorAtRun,
}
