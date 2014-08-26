/// TODO
pub struct Stack<T> {
    elements: Vec<T>
}

/// TODO
impl<T> Stack<T> {
    /// TODO
    pub fn new() -> Stack<T> {
        Stack {
            elements: Vec::new()
        }
    }

    /// TODO
    pub fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_new() {
        // Initialize the test stack.
        let stack: Stack<int> = Stack::new();

        // Manually construct the Stack object expected from Stack::new().
        let expected_stack: Stack<int> = Stack { elements: Vec::new() };

        // Verify that Stack::new() produces a new empty Stack instance.
        assert_eq!(stack, expected_stack);
    }

    #[test]
    fn test_push() {
        // Initialize a test stack.
        let mut stack: Stack<int> = Stack::new();

        // Push an element onto the stack.
        stack.push(10);
    }
}
