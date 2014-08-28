use stack::Stack;

/// A stack, backed by a `Vec`.
pub struct VecStack<T> {
    elements: Vec<T>
}

impl<T> VecStack<T> {
    /// Creates a new vector-backed stack.
    pub fn new() -> VecStack<T> {
        VecStack { elements: Vec::new() }
    }
}

impl<T> Stack<T> for VecStack<T> {
    fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> uint {
        self.elements.len()
    }
}
