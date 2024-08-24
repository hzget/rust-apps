pub struct Post {
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DraftPost, Post};

    #[test]
    fn new() {
        assert_eq!(
            DraftPost {
                content: String::new()
            },
            Post::new()
        );
    }

    #[test]
    fn content() {
        let mut post = Post::new();
        post.add_text("hello");
        assert_eq!("hello", post.request_review().approve().content());
    }
}
