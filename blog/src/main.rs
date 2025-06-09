use blog;

fn main() {
    let mut post = blog::new(); // post is of type Draft

    post.add_text("good morning");

    let post = post.request_review(); // post is of type PendingReview
    let post = post.approve(); // post is of type Published
    assert_eq!("good morning", post.content());
}
