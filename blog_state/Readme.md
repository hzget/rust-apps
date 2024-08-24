
blog_state
===

`blog_state` is a crate that works for a post:
create a draft, request a review and then publish it.

It intends to show how to implement functionalities using a
**state pattern** (object-oriented design pattern).

There is another implementation: [blog](../blog) which
does not use state pattern but rather type sytem of rust.
It encodes `state` and `behavior` as rust types.

Requirements
---

1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts cannot accidentally be published.

User case
---

```rust
use blog_state::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Point of view
---

From a client's point of view, he observes that a `post` object
does something. But in the underlying, a `state` object
experiences transitions and does something.

Design
---

A post will experience ***state*** transitions:

```bash
draft --> review --> published
```

The rules of the blog post workflow will live in state objects.
Each state will have its own logic rules. For example,
a published state object -- a post object in the "published" status --
can print the post contents. Others could not.

All states implement a `State` trait which list all supported
methods for a concret state. In other words, all states objects
share the same methods but they have different implementation,
i.e., different behavior.

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
impl State for Draft         {/* Draft implementation */}
impl State for PendengReview {/* PendingReview implementation */}
impl State for Published     {/* Published implementation */}
```

A state contains its "rules"
---

We design a struct `Post` containing post contents and its current state.
We can take a post instance as a `state` object because one post
has one and only one state at one time.

The `state` object behaves in this way:
each time a post calls a method, it will delegate to a method of its state.

For example, post.content() delegates to a content method defined on
its state:

```rust
pub struct Post {
    state: Option<Box<dyn State>>, // <--- a state object is a trait object
    content: String,
}

impl Post {
    pub fn content(&self) -> &str {
        match &self.state {
            Some(s) => s.content(self),  // <---- it contains rules in requirement
            None => "",
        }
    }
}
```

The behavior of post.content() depends on the behavior of its
current state.

state transitions
---

The state "transition" works in the same way. The actual transition
is done by the state object itself.

```rust
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
}
```

