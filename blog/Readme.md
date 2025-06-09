blog
===

`blog` is a crate that works for a post:
create a draft, request a review and then publish it.

It intends to show how to implement functionalities using
the **rust type system**. It is different from the
**state pattern** (object-oriented design pattern)
used in [blog_state](../blog_state).

Rather than encapsulating the states and transitions completely
so outside code has no knowledge of them, we'll
**encode the states into different types**. 
Consequently, Rust’s type checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

Requirements
---

1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts cannot accidentally be published.

User case
---

```rust
use blog;

fn main() {
    let mut post = blog::new(); // post is of type Draft

    post.add_text("good morning");

    let post = post.request_review(); // post is of type PendingReview
    let post = post.approve(); // post is of type Published
    assert_eq!("good morning", post.content());
}
```

Point of view
---

From a client's point of view, he observes that a `post` object
performed some actions. In the underlying, the `post` object
experienced type transformations, and did specific jobs in
corresponding types of instance.

The underlying mechanism is similar to a `state` object
experiences state transitions.

Design
---

A post will experience ***type*** transformation:

```bash
Draft --> PendingReview --> Published
```

The rules of the blog post workflow will live in **type instance** (i.e., object).
Each type will have its own logic rules. For example,
a ***Published*** instance can print the post contents. Others could not.

Features
---

Compared with `state pattern`, our design have following features:

* Encoding States and Behavior as Types
* Avoid invalid state transitions at compile time

