//! `stack` is a module that supports push() and pop() methods.
//!
//! Features:
//!
//!    * First-In-Last-Out
//!    * generic
//!    * implement Iterator
//!    * size is checkable
//!    * able to peek at the upcomming item
//!
//! Example:
//!
//! ```rust
//!    use list::stack::Stack;
//!
//!    let mut list = Stack::new();
//!    list.push(1);
//!    list.push(2);
//!    println!("{:?}", list.length()); // 2
//!    println!("{:?}", list.pop()); // Some(2)
//!    println!("{:?}", list.pop()); // Some(1)
//!    println!("{:?}", list.pop()); // None
//!
//!    let mut list = Stack::new();
//!    list.push("yes");
//!    list.push("no");
//!    assert_eq!(list.peek(), Some("no").as_ref());
//!    println!("{:?}", list.peek()); // Some("no")
//!    println!("{:?}", list.pop()); // Some("no")
//!    println!("{:?}", list.pop()); // Some("yes")
//!    println!("{:?}", list.pop()); // None
//!
//!    let mut list = Stack::new();
//!    list.push("yes");
//!    list.push("no");
//!    for x in list {
//!        println!("{:?}", x)
//!    }
//!    // Output:
//!    //     "no"
//!    //     "yes"
//! ```
//!
use super::Node;

/// A List that implements a functionaliy of a `Stack` (FirstInLastOut)
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    length: i32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }
    pub fn push(&mut self, data: T) {
        let node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.length += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            self.length -= 1;
            x.data
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().take().map(|x| &x.data)
    }
    pub fn length(&self) -> i32 {
        self.length
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek() {
        let list: Stack<i32> = Stack::new();
        assert_eq!(list.peek(), None);

        let mut list = Stack::new();
        list.push(401);
        list.push(200);
        assert_eq!(list.peek(), Some(200).as_ref());
        assert_eq!(list.peek(), Some(200).as_ref());
        list.pop();
        list.pop();
        assert_eq!(list.peek(), None);

        let mut list = Stack::new();
        list.push("UnAuthorized");
        list.push("OK");
        assert_eq!(list.peek(), Some("OK").as_ref());
    }

    #[test]
    fn push_and_pop() {
        let mut list: Stack<i32> = Stack::new();
        assert_eq!(list.pop(), None);

        let mut list = Stack::new();
        assert_eq!(list.length(), 0);
        list.push(401);
        list.push(200);
        assert_eq!(list.length(), 2);
        assert_eq!(list.pop(), Some(200));
        assert_eq!(list.pop(), Some(401));
        assert_eq!(list.length(), 0);
        assert_eq!(list.pop(), None);

        let mut list = Stack::new();
        list.push("UnAuthorized");
        list.push("OK");
        assert_eq!(list.pop(), Some("OK"));
        assert_eq!(list.pop(), Some("UnAuthorized"));
    }

    #[test]
    fn iteration() {
        let mut list = Stack::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut i = 3;
        for x in list {
            assert_eq!(x, i);
            i -= 1;
        }

        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        let list = list.filter(|x| x % 2 == 0);
        i = 4;
        for x in list {
            assert_eq!(x, i);
            i -= 2;
        }
    }
}
