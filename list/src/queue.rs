//! `queue` is a module that supports enqueue() and dequeue() methods.
//!
//! Features:
//!
//!    * First-In-First-Out
//!    * generic
//!
//! Example:
//!
//! ```rust
//! use list::queue::Queue;
//!
//! let mut list = Queue::new();
//! list.enqueue(1);
//! list.enqueue(2);
//! list.enqueue(3);
//!
//! println!("{:?}", list.dequeue()); // Some(1)
//! println!("{:?}", list.dequeue()); // Some(2)
//! println!("{:?}", list.dequeue()); // Some(3)
//! println!("{:?}", list.dequeue()); // None
//! ```

use super::Node;

/// FIFI Queue
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        let node = Node { data, next: None };
        let raw_tail = Box::into_raw(Box::new(node));

        unsafe {
            if let Some(x) = self.tail {
                (*x).next = Some(Box::from_raw(raw_tail));
            } else {
                self.head = Some(Box::from_raw(raw_tail));
            }
        }
        self.tail = Some(raw_tail);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            if let Some(next) = x.next {
                self.head = Some(next);
            } else {
                self.tail = None;
            }
            x.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_dequeue() {
        let mut list = Queue::new();
        list.enqueue(401);
        list.enqueue(200);
        assert_eq!(list.dequeue(), Some(401));
        assert_eq!(list.dequeue(), Some(200));

        let mut list = Queue::new();
        list.enqueue("UnAuthorized");
        list.enqueue("OK");
        assert_eq!(list.dequeue(), Some("UnAuthorized"));
        assert_eq!(list.dequeue(), Some("OK"));
    }
}
