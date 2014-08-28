extern crate adt;

use adt::stack::{
    Stack,
    VecStack
};

fn main() {
    // Initialize a VecStack stack.
    let mut stack: VecStack<int> = VecStack::new();

    // Push some elements onto the stack.
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Pop some elements off the stack.
    println!("{}", stack.pop());

    // Peek some elements from the stack.
    println!("{}", stack.peek());
}
