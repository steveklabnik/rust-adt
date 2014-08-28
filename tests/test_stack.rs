extern crate adt;

#[cfg(test)]
mod test {
    use adt::{
        VecStack,
        Stack
    };

    #[test]
    fn test_new_vec_stack() {
        // Initialize the test stack.
        let stack: VecStack<int> = VecStack::new();

        // Verify that the stack is empty.
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_vec_stack() {
        // Initialize a test stack.
        let mut stack: VecStack<int> = VecStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the stack contains 3 elements.
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_push_then_pop_vec_stack() {
        // Initialize a test stack.
        let mut stack: VecStack<int> = VecStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the pop() function works correctly.
        assert_eq!(stack.pop(), Some(22));
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_push_then_peek_vec_stack() {
        // Initialize a test stack.
        let mut stack: VecStack<int> = VecStack::new();

        // Push some elements onto the stack.
        stack.push(1);
        stack.push(43);
        stack.push(22);

        // Verify that the peek() function works correctly.
        assert_eq!(stack.peek(), Some(&22));
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_pop_until_empty_vec_stack() {
        // Initialize a test stack.
        let mut stack: VecStack<int> = VecStack::new();

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
    fn test_pop_empty_vec_stack() {
        // Initialize a test stack.
        let mut stack: VecStack<int> = VecStack::new();

        // Pop a non-existing element off the stack.
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_empty_vec_stack() {
        // Initialize a test stack.
        let stack: VecStack<int> = VecStack::new();

        // Peek a non-existing element from the stack.
        assert_eq!(stack.peek(), None);
    }
}
