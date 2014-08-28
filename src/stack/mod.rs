/*! Stack ADT implementations, including DListStack and VecStack

`adt::stack` provides adt's Stack ADT traits and implementations.

*/

pub use self::dlist_stack::DListStack;
pub use self::stack::Stack;
pub use self::vec_stack::VecStack;

mod dlist_stack;
mod stack;
mod vec_stack;
