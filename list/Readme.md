List
===

`list` is a library that provides list-related utilities.

* [x] stack
* [ ] queue

Stack Functionality
---

***Bad design***

```rust
let list = LinkedList::new();
list.push(Node::new(1));
list.push(Node::new(2));
let node_a = list.pop();
let node_b = list.pop();
println!("{:?}", node_a.unwrap().data);
println!("{:?}", node_b.unwrap().data);
```

***Good Design***

```rust
let mut list = List::new();
list.push(1);
list.push(2);
println!("{:?}", list.pop()); // Some(2)
println!("{:?}", list.pop()); // Some(1)
println!("{:?}", list.pop()); // None
```

A ***GOOD*** design should be:

* simple
* intuitive
* user-friendly
* hide implementation details

