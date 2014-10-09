/// A queue.
pub trait Queue<T> {
    /// Enqueue an element to the rear of the queue.
    fn enqueue(&mut self, elem: T);
    /// Dequeues an element from the front of the queue and returns it.
    /// Returns None if the queue is empty.
    fn dequeue(&mut self) -> Option<T>;
    /// Returns `true` if the queue is empty.
    fn is_empty(&self) -> bool;
    /// Returns the size of the queue.
    fn size(&self) -> uint;
    /// Provides a reference to the object at the front of the queue without
    /// removing it.
    /// Returns None if the queue is empty.
    fn front<'a>(&'a self) -> Option<&'a T>;
}
