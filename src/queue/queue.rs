/// A queue.
pub trait Queue<T> {
    /// Queue an element to the queue.
    fn enqueue(&mut self, elem: T);
    /// Queues an element from the queue and returns it, or None if the queue is empty.
    fn dequeue(&mut self) -> Option<T>;
    /// Returns `true` if the `Queue` is empty.
    fn is_empty(&self) -> bool;
    /// Returns the size of the `Queue`.
    fn size(&self) -> uint;
}
