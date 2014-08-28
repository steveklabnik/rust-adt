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

    // Peek an element from the stack.
    println!("{}", stack.peek());

    // Pop some elements off the stack.
    println!("{}", stack.pop());
    println!("{}", stack.pop());
    println!("{}", stack.pop());

    // Attempt to pop an element from the empty stack.
    println!("{}", stack.pop());
}
