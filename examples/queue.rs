extern crate adt;

use adt::queue::{
    Queue,
    DListQueue
};

fn main() {
    // Initialize a DListQueue queue.
    let mut queue: DListQueue<int> = DListQueue::new();

    // Queue some elements to the queue.
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    // Dequeue some elements off the queue.
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());

    // Attempt to dequeue from the empty stack.
    println!("{}", queue.dequeue());
}
