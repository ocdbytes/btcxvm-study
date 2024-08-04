use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpCodeErrors {
    #[error("Value missing in stack while executing the opcode : {0}")]
    MissingValue(String),
    #[error("Error converting the input into the range of -2^31 to 2^31 - 1 (i32)")]
    NumberNotInRange,
    #[error("Verification failed")]
    OpVerifyFailed,
}

#[derive(Error, Debug)]
pub enum InputParsingError {
    #[error("Not able to parse the inputs provided to the vm.")]
    InputParsingErrorAtRun,
}
