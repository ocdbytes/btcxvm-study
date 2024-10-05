pub mod executor;

#[cfg(test)]
mod flow_tests;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    pub elements: Vec<String>,
    pub length: i32,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            elements: vec![],
            length: 0,
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        if self.length >= 1 {
            self.length -= 1;
        }
        self.elements.pop()
    }

    pub fn push(&mut self, item: String) {
        self.length += 1;
        self.elements.push(item);
    }

    pub fn push_to_top(&mut self, item: String) {
        self.length += 1;
        self.elements.insert(0, item);
    }

    pub fn pop_from_top(&mut self) -> Option<String> {
        if self.length >= 1 {
            self.length -= 1;
        } else {
            return None;
        }
        Some(self.elements.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn read_ele_from_top(&self, number: i32) -> Option<&String> {
        self.elements.get(number as usize)
    }

    #[cfg(test)]
    pub fn stack_from(vec: Vec<String>) -> Stack {
        let mut stack = Stack::new();
        for i in vec {
            stack.push_to_top(i);
        }
        stack
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pop_element() {
        let mut stk = Stack::new();
        stk.push("1".to_string());
        stk.push("2".to_string());
        stk.push("3".to_string());

        let pop_res = stk.pop().unwrap();
        assert_eq!(pop_res, "3");
    }

    #[test]
    fn push_element() {
        let mut stk = Stack::new();
        stk.push("1".to_string());
        stk.push("2".to_string());
        stk.push("3".to_string());

        let mut pop_res = stk.pop().unwrap();
        assert_eq!(pop_res, "3");
        pop_res = stk.pop().unwrap();
        assert_eq!(pop_res, "2");
        pop_res = stk.pop().unwrap();
        assert_eq!(pop_res, "1");
    }

    #[test]
    fn is_empty() {
        let stk = Stack::new();
        assert!(stk.is_empty());
    }
}
