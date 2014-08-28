/*! Queue ADT implementations, currently only DListQueue

`adt::queue` provides adt's Queue ADT traits and implementations.

*/

pub use self::dlist_queue::DListQueue;
pub use self::queue::Queue;

mod dlist_queue;
mod queue;
