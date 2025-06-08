
blog\_state
===

`blog_state` is a crate that works for a post:
create a draft, request a review and then publish it.

It intends to show how to implement functionalities using a
**state pattern** (object-oriented design pattern).

There is another implementation: [blog](../blog) which
does not use state pattern but rather rust type system.
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
performed some actions. But in the underlying, a `state` object
did the actual jobs and then experienced a state transition.

State
---

A post will experience ***state*** transitions:

```bash
Draft --> PendingReview --> Published
```

The `State` trait defines the interface of the ***state***s.

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

Each state has its own behavior.

Design
---

We design a struct `Post` containing post contents and its current state.
We can take a post instance as a `state` object because one post
has one and only one state at one time.

The `state` object behaves in this way:
each time a post calls a method, it will delegate to a method of its state.

In other words, the method of `post` is the "frontend" used by the
user client whereas the method of `state` is the "backend" which actually
does the job.

Suppose a ***Draft*** post requests a review:

```rust
pub struct Post {
    state: Option<Box<dyn State>>, // <--- a state object is a trait object
    content: String,
}

// "frontend"
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // 1. delegates to State::request_review()
            // 2. experience a state transition
            self.state = Some(s.request_review());
        }
    }
}

// "backend"
impl State for Draft {
    // 1. trigger a "review" request
    // 2. return a new state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

```

