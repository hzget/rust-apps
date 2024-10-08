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
            match self.tail {
                Some(x) => {
                    (*x).next = Some(Box::from_raw(raw_tail));
                }
                None => {
                    self.head = Some(Box::from_raw(raw_tail));
                }
            }
        }
        self.tail = Some(raw_tail);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            match x.next {
                Some(next) => self.head = Some(next),
                None => self.tail = None,
            }
            x.data
        })
    }
}

impl<T> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_dequeue() {
        let mut list: Queue<i32> = Queue::new();
        assert_eq!(list.dequeue(), None);

        let mut list = Queue::new();
        list.enqueue(401);
        list.enqueue(200);
        assert_eq!(list.dequeue(), Some(401));
        assert_eq!(list.dequeue(), Some(200));
        assert_eq!(list.dequeue(), None);

        let mut list = Queue::new();
        list.enqueue("UnAuthorized");
        list.enqueue("OK");
        assert_eq!(list.dequeue(), Some("UnAuthorized"));
        assert_eq!(list.dequeue(), Some("OK"));
    }

    #[test]
    fn iteration() {
        let mut list = Queue::new();
        list.enqueue(0);
        list.enqueue(1);
        list.enqueue(2);
        list.enqueue(3);
        let mut i = 0;
        for x in list {
            assert_eq!(x, i);
            i += 1;
        }

        let mut list = Queue::new();
        list.enqueue(1);
        list.enqueue(2);
        list.enqueue(3);
        list.enqueue(4);
        let list = list.filter(|x| x % 2 == 0);
        i = 2;
        for x in list {
            assert_eq!(x, i);
            i += 2;
        }
    }
}
