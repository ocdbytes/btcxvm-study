// Stack implementation for the btc vm

#[derive(Debug,Clone)]
pub struct Stack {
    pub elements: Vec<String>,
    pub length: u32
}

impl Stack {
    pub fn new() -> Stack {
        Stack {elements: vec![], length: 0 }
    }

    pub fn pop(&mut self) -> Option<String> {
        if self.length > 1 {
            self.length -= 1;
        }
        self.elements.pop()
    }

    pub fn push(&mut self, item : String) {
        self.length += 1;
        self.elements.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
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
        assert_eq!(stk.is_empty(), true);
    }

}