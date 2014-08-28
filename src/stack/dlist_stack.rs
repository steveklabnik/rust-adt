use std::collections::dlist::DList;
use std::collections::Deque;

use stack::Stack;

/// TODO
pub struct DListStack<T> {
    /// TODO
    list: DList<T>
}

/// TODO
impl<T> DListStack<T> {
    /// TODO
    pub fn new() -> DListStack<T> {
        DListStack { list: DList::new() }
    }
}

/// TODO
impl<T> Stack<T> for DListStack<T> {
    /// TODO
    fn push(&mut self, elem: T) {
        self.list.push(elem);
    }

    /// TODO
    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    /// TODO
    fn peek(&self) -> Option<&T> {
        self.list.back()
    }

    /// TODO
    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// TODO
    fn size(&self) -> uint {
        self.list.len()
    }
}
