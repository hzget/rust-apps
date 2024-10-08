//! `list` is a library that supports stack and queue.

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub mod queue;
pub mod stack;
