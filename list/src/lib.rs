//! `list` is a library that supports stack.

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub mod stack;

