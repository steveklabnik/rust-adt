/// TODO
pub trait List<T> {
    /// TODO
    fn append(&mut self, elem: T);
    /// TODO
    fn prepend(&mut self, elem: T);
    /// TODO
    fn head(&self) -> Option<&T>;
    /// TODO
    fn tail(&self) -> Self;
    /// TODO
    fn is_empty(&self) -> bool;
    /// TODO
    fn size(&self) -> uint;
}
