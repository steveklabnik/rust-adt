extern crate adt;

#[cfg(test)]
mod test {
    use adt::{
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
