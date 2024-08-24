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
