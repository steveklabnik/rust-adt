use std::fmt::Show;

/// TODO
pub trait List<T> {
    /// TODO
    fn append(&mut self, elem: T);
    /// TODO
    fn prepend(&mut self, elem: T);
    /// TODO
    fn head(&self) -> Option<T>;
    /// TODO
    fn tail(&self) -> Option<T>;
    /// TODO
    // TODO: Implement this with indexing instead.
    //fn get(&self, index: uint) -> Option<T>;
    /// TODO
    //fn insert(&mut self, index: uint);
    /// TODO
    fn is_empty(&self) -> bool;
    /// TODO
    fn size(&self) -> uint;
}

/// TODO
#[deriving(Clone, Show)]
struct Node<T> {
    /// TODO
    value: T,
    /// TODO
    next: Option<Box<Node<T>>>,
    /// TODO
    prev: Option<Box<Node<T>>>
}

/// TODO
#[deriving(Show)]
pub struct DList<T> {
    /// TODO
    head: Option<Box<Node<T>>>,
    /// TODO
    tail: Option<Box<Node<T>>>,
    /// TODO
    size: uint
}

impl<T: Clone + Show> List<T> for DList<T> {
    /// TODO
    fn append(&mut self, elem: T) {
        let prev_tail = self.tail.clone();
        let new_tail: Node<T> = Node {
                value: elem,
                next: None,
                prev: prev_tail
            };
        self.tail = Some(box new_tail);
        self.size += 1;
    }

    /// TODO
    fn prepend(&mut self, elem: T) {
        let prev_head = self.head.clone();
        let new_head: Node<T> = Node {
                value: elem,
                next: prev_head,
                prev: None
            };
        self.head = Some(box new_head);
        self.size += 1;
    }

    /// TODO
    fn head(&self) -> Option<T> {
        match self.head.clone() {
            Some(node) => Some(node.value),
            None       => None
        }
    }

    /// TODO
    fn tail(&self) -> Option<T> {
        match self.tail.clone() {
            Some(node) => Some(node.value),
            None       => None
        }
    }

    /// TODO
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// TODO
    fn size(&self) -> uint {
        self.size
    }
}

impl<T> DList<T> {
    pub fn new() -> DList<T> {
        DList { head: None, tail: None, size: 0 }
    }
}
