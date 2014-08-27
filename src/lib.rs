#![deny(non_camel_case_types)]

pub use list::{
    DList,
    List
};
pub use stack::{
    ArrayStack,
    Stack
};

mod list;
mod stack;
