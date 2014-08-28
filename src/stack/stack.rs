/// A stack.
pub trait Stack<T> {
    /// Add an element to the top of the stack.
    fn push(&mut self, elem: T);
    /// Removes the element on the top of the stack and returns it, or None if the stack is empty.
    fn pop(&mut self) -> Option<T>;
    /// Returns a reference to the element on the top of the stack.
    fn peek(&self) -> Option<&T>;
    /// Returns `true` if the `Stack` is empty.
    fn is_empty(&self) -> bool;
    /// Returns the size of the `Stack`.
    fn size(&self) -> uint;
}
