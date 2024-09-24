//! `list` is a library that supports push() and pop() methods.
//!
//! Features:
//!
//!    * First-In-Last-Out
//!    * generic
//!    * implement Iterator
//!    * size is checkable
//!
//! Example:
//!
//! ```rust
//!    use list::LinkedList;
//!
//!    let mut list = LinkedList::new();
//!    list.push(1);
//!    list.push(2);
//!    println!("{:?}", list.length()); // 2
//!    println!("{:?}", list.pop()); // Some(2)
//!    println!("{:?}", list.pop()); // Some(1)
//!    println!("{:?}", list.pop()); // None
//!
//!    let mut list = LinkedList::new();
//!    list.push("yes");
//!    list.push("no");
//!    println!("{:?}", list.pop()); // Some("no")
//!    println!("{:?}", list.pop()); // Some("yes")
//!    println!("{:?}", list.pop()); // None
//!
//!    let mut list = LinkedList::new();
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

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// A List that implements a functionaliy of a `Stack` (FirstInLastOut)
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: i32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
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
    pub fn length(&self) -> i32 {
        self.length
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut list = LinkedList::new();
        assert_eq!(list.length(), 0);
        list.push(401);
        list.push(200);
        assert_eq!(list.length(), 2);
        assert_eq!(list.pop(), Some(200));
        assert_eq!(list.pop(), Some(401));
        assert_eq!(list.length(), 0);

        let mut list = LinkedList::new();
        list.push("UnAuthorized");
        list.push("OK");
        assert_eq!(list.pop(), Some("OK"));
        assert_eq!(list.pop(), Some("UnAuthorized"));
    }

    #[test]
    fn iteration() {
        let mut list = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut i = 3;
        for x in list {
            assert_eq!(x, i);
            i -= 1;
        }

        let mut list = LinkedList::new();
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
