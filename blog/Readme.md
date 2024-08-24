blog
===

`blog` is a crate that works for a post:
create a draft, request a review and then publish it.

It intends to show how to implement functionalities using
**type system of rust** instead of using
**state pattern** (object-oriented design pattern), just as
how [blog_state](../blog_state) does.

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
use blog::Post;

const TEXTSTR: &str = "I ate a salad for lunch today";
fn main() {
    let mut post = Post::new();
    post.add_text(TEXTSTR);

    // error[E0599]:
    // no method named `content` found for struct `DraftPost`
    // in the current scope
    //
    // In fact, post now is of type `DraftPost`, not `Post`
    // assert_eq!("", post.content());

    let post = post.request_review(); // post is of type PendingReviewPost
    let post = post.approve(); // post is of type Post
    assert_eq!(TEXTSTR, post.content());
}
```

Point of view
---

From a client's point of view, he observes that a `post` object
does something. But in the underlying, the "post" object
experiences type transformations and does somthing.

The underlying mechanism is similar to a `state` object
experiences state transitions and does something.

Design
---

A post will experience ***type*** transformation:

```bash
DraftPost --> PendingReviewPost --> Post
```

The rules of the blog post workflow will live in **type instance** (i.e., object).
Each type will have its own logic rules. For example,
a Post instance can print the post contents. Others could not.

Features
---

Compared with `state pattern`, our design have following features:

* Encoding States and Behavior as Types
* Implementing Transitions as Transformations into Different Types
* Make invalid states and transitions into compile time errors

