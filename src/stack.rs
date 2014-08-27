/// TODO
pub trait Stack<T> {
    /// TODO
    fn push(&mut self, elem: T);
    /// TODO
    fn pop(&mut self) -> Option<T>;
    /// TODO
    fn peek(&self) -> Option<&T>;
    /// TODO
    fn is_empty(&self) -> bool;
    /// TODO
    fn size(&self) -> uint;
}

/// TODO
pub struct ArrayStack<T> {
    /// TODO
    elements: Vec<T>
}

/// TODO
impl<T> ArrayStack<T> {
    /// TODO
    pub fn new() -> ArrayStack<T> {
        ArrayStack { elements: Vec::new() }
    }
}

/// TODO
impl<T> Stack<T> for ArrayStack<T> {
    /// TODO
    fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }

    /// TODO
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// TODO
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// TODO
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// TODO
    fn size(&self) -> uint {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArrayStack,
        Stack
    };

    #[test]
    fn test_new_array_stack() {
        // Initialize the test stack.
        let stack: ArrayStack<int> = ArrayStack::new();

        // Verify that the stack is empty.
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_array_stack() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the stack contains 3 elements.
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_push_then_pop_array_stack() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the pop() function works correctly.
        assert_eq!(stack.pop().unwrap(), 22);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_push_then_peek_array_stack() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the peek() function works correctly.
        assert_eq!(stack.peek().unwrap(), &22);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_pop_until_empty_array_stack() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Pop the elements off the stack.
        stack.pop();
        stack.pop();
        stack.pop();

        // Verify that the stack is now empty.
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_pop_empty_array_stack() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Pop a non-existing element off the stack.
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_empty_array_stack() {
        // Initialize a test stack.
        let stack: ArrayStack<int> = ArrayStack::new();

        // Peek a non-existing element from the stack.
        assert_eq!(stack.peek(), None);
    }
}
