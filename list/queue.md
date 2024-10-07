Queue
===

API
---

```rust
let mut list = Queue::new();
list.enqueue(1);
list.enqueue(2);
list.enqueue(3);

println!("{:?}", list.dequeue()); // Some(1)
println!("{:?}", list.dequeue()); // Some(2)
println!("{:?}", list.dequeue()); // Some(3)
println!("{:?}", list.dequeue()); // None
```

Technique
---

source code: [src/queue.rs](./src/queue.rs)

* wrap the pointer in `Option`
* generic
* Iterator
* raw pointer
* unsafe code

