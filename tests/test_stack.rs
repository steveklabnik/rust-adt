extern crate adt;

#[cfg(test)]
mod test {
    use adt::{
        ArrayStack,
        Stack
    };

    #[test]
    fn test_stack() {
        // Initialize the test stack.
        let mut stack: ArrayStack<int> = Stack::new();

        // Add some values to the stack.
        stack.push(10);
        stack.push(4);
        stack.push(3);
    }
}
