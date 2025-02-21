const WARNING: &str = "Warning: You've used up over 75% of your quota!";
const WARNING_URGENT: &str = "Urgent warning: You've used up over 90% of your quota!";
const WARNING_FATAL: &str = "Error: You are over your quota!";

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send(WARNING_FATAL);
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(WARNING_URGENT);
        } else if percentage_of_max >= 0.75 {
            self.messenger.send(WARNING);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow()[0], WARNING);
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(90);

        assert_eq!(mock_messenger.sent_messages.borrow()[0], WARNING_URGENT);
    }

    #[test]
    fn it_sends_an_over_100_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(100);

        assert_eq!(mock_messenger.sent_messages.borrow()[0], WARNING_FATAL);
    }
}
