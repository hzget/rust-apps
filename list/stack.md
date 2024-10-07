Stack
===

API
---

```rust
let mut list = Stack::new();
list.push(1);
list.push(2);
println!("{:?}", list.pop()); // Some(2)
println!("{:?}", list.pop()); // Some(1)
println!("{:?}", list.pop()); // None
```

Technique
---

source code: [src/stack.rs](./src/stack.rs)

* wrap the pointer in `Option`
* generic
* Iterator

