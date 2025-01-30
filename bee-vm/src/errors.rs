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
    #[error("Error : {0}")]
    InvalidHex(String),
    #[error("Error : {0}")]
    UnbalancedControlFlow(String),
    #[error("Error : {0}")]
    InvalidValue(String),
    #[error("OP_RETURN called")]
    OpReturn,
    #[error("OP_RESERVED called")]
    OpReserved,
    #[error("Stack Empty")]
    StackEmpty,
    #[error("Unknown Opcode")]
    UnknownOpcode,
    #[error("Invalid Public Key")]
    InvalidPublicKey,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Error creating message")]
    MessageCreationError,
}

#[derive(Error, Debug)]
pub enum InputParsingError {
    #[error("Not able to parse the inputs provided to the vm.")]
    InputParsingErrorAtRun,
}
