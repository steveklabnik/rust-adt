/// TODO
pub trait Stack<T> {
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}

/// TODO
pub struct ArrayStack<T> {
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
}

#[cfg(test)]
mod tests {
    use super::{
        ArrayStack,
        Stack
    };

    #[test]
    fn test_new() {
        // Initialize the test stack.
        let stack: ArrayStack<int> = ArrayStack::new();

        // Manually construct the Stack object expected from Stack::new().
        let expected_stack: ArrayStack<int> = ArrayStack { elements: Vec::new() };

        // Verify that Stack::new() produces a new empty Stack instance.
        //assert_eq!(stack, expected_stack);
    }

    #[test]
    fn test_push() {
        // Initialize a test stack.
        let mut stack: ArrayStack<int> = ArrayStack::new();

        // Push an element onto the stack.
        stack.push(10);
    }
}
