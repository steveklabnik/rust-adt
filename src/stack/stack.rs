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
