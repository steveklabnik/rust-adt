/// TODO
pub trait Stack<T> {
    /// TODO
    fn push(&mut self, elem: T);
    /// TODO
    fn pop(&mut self) -> Option<T>;
    /// TODO
    // TODO: Find a way to make this Option<T>.
    fn peek(&self) -> Option<&T>;
    /// TODO
    fn is_empty(&self) -> bool;
    /// TODO
    fn size(&self) -> uint;
}

/// TODO
pub struct VecStack<T> {
    /// TODO
    elements: Vec<T>
}

/// TODO
impl<T> VecStack<T> {
    /// TODO
    pub fn new() -> VecStack<T> {
        VecStack { elements: Vec::new() }
    }
}

/// TODO
impl<T> Stack<T> for VecStack<T> {
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
