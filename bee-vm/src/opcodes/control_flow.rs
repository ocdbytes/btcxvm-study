use crate::errors::OpCodeErrors;
use crate::stack::Stack;

pub struct ControlFlow {
    if_stack: Vec<bool>,
}

impl ControlFlow {
    pub fn new() -> Self {
        ControlFlow {
            if_stack: Vec::new(),
        }
    }

    pub fn op_if(&mut self, stack: &mut Stack) -> Result<(), OpCodeErrors> {
        if self.should_execute() {
            let condition = match stack.pop_from_top() {
                Some(val) => val,
                None => return Err(OpCodeErrors::StackEmpty),
            };
            let execute = condition != "0" && condition.to_lowercase() != "false";
            self.if_stack.push(execute);
        } else {
            self.if_stack.push(false);
        }
        Ok(())
    }

    pub fn op_else(&mut self) -> Result<(), OpCodeErrors> {
        if let Some(last) = self.if_stack.last_mut() {
            *last = !*last;
            Ok(())
        } else {
            Err(OpCodeErrors::UnbalancedControlFlow(
                "OP_ELSE without OP_IF".to_string(),
            ))
        }
    }

    pub fn op_endif(&mut self) -> Result<(), OpCodeErrors> {
        if self.if_stack.pop().is_some() {
            Ok(())
        } else {
            Err(OpCodeErrors::UnbalancedControlFlow(
                "OP_ENDIF without OP_IF".to_string(),
            ))
        }
    }

    pub fn should_execute(&self) -> bool {
        self.if_stack.is_empty() || *self.if_stack.last().unwrap()
    }
}

impl Default for ControlFlow {
    fn default() -> Self {
        Self::new()
    }
}
