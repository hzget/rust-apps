use limit_tracker::*;
use std::cell::RefCell;
use std::ops::AddAssign;

struct Game<'a> {
    counter: RefCell<Counter<'a>>,
}

impl<'a> Game<'a> {
    fn new(counter: RefCell<Counter<'a>>) -> Self {
        Game { counter }
    }

    fn reward(&self) {
        *self.counter.borrow_mut() += 1;
        // - actual code for rewarding -
    }
}

fn main() {
    let msnger = MyMessenger;
    let a = Game::new(Counter::new(&msnger, 10));
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

struct Counter<'a> {
    count: usize,
    tracker: LimitTracker<'a, MyMessenger>,
}

impl<'a> Counter<'a> {
    fn new(msnger: &'a MyMessenger, max: usize) -> RefCell<Self> {
        RefCell::new(Counter {
            count: 0,
            tracker: LimitTracker::new(msnger, max),
        })
    }
}

impl<'a> AddAssign<usize> for Counter<'a> {
    fn add_assign(&mut self, other: usize) {
        self.count = self.count + other;
        self.tracker.set_value(self.count);
    }
}

struct MyMessenger;

impl Messenger for MyMessenger {
    fn send(&self, message: &str) {
        println!("{}", message);
    }
}
