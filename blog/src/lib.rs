pub struct Draft {
    content: String,
}

pub struct PendingReview {
    content: String,
}

pub struct Published {
    content: String,
}

pub fn new() -> Draft {
    Draft {
        content: String::new(),
    }
}

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
        }
    }
}

impl PendingReview {
    pub fn approve(self) -> Published {
        Published {
            content: self.content,
        }
    }
}

impl Published {
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use crate::new;

    #[test]
    fn content() {
        let mut post = new();
        post.add_text("hello");
        assert_eq!("hello", post.request_review().approve().content());
    }
}
