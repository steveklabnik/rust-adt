use std::collections::dlist::DList;
use std::collections::Deque;

use stack::Stack;

/// A stack, backed by a doubly linked list.
pub struct DListStack<T> {
    list: DList<T>
}

impl<T> DListStack<T> {
    /// Creates a new `Stack`, backed by a `DList`.
    pub fn new() -> DListStack<T> {
        DListStack { list: DList::new() }
    }
}

impl<T> Stack<T> for DListStack<T> {
    fn push(&mut self, elem: T) {
        self.list.push(elem);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.list.back()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn size(&self) -> uint {
        self.list.len()
    }
}
