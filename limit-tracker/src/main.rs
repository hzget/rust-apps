use limit_tracker::*;

struct Game<'a, T>
where
    T: limit_tracker::Messenger,
{
    tracker: LimitTracker<'a, T>,
}

impl<'a, T> Game<'a, T>
where
    T: limit_tracker::Messenger,
{
    fn new(tracker: LimitTracker<'a, T>) -> Self {
        Game { tracker }
    }

    fn reward(&mut self) {
        self.tracker += 1;
        // - actual code for rewarding -
    }
}

fn main() {
    let msnger = MyMessenger;
    let mut a = Game::new(LimitTracker::new(&msnger, 10));
    a.reward();
    a.reward();
    a.reward();
    a.reward();
    a.reward();
    a.reward();
    a.reward();

    a.reward(); // Warning: You've used up over 75% of your quota!

    a.reward(); // Urgent warning: You've used up over 90% of your quota!
    a.reward(); // Error: You are over your quota!
}

struct MyMessenger;

impl Messenger for MyMessenger {
    fn send(&self, message: &str) {
        println!("{}", message);
    }
}
