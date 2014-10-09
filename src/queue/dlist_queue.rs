use std::collections::dlist::DList;
use std::collections::Deque;

use queue::Queue;

/// A queue, backed by a doubly linked list.
pub struct DListQueue<T> {
    list: DList<T>
}

impl<T> DListQueue<T> {
    /// Creates a new `Queue`, backed by a `DList`.
    pub fn new() -> DListQueue<T> {
        DListQueue { list: DList::new() }
    }
}

impl<T> Queue<T> for DListQueue<T> {
    fn enqueue(&mut self, elem: T) {
        self.list.push(elem);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn size(&self) -> uint {
        self.list.len()
    }

    fn front<'a>(&'a self) -> Option<&'a T> {
        self.list.front()
    }
}
