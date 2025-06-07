# limit-tracker

`limit-tracker` create a library that tracks a value against a maximum value
and sends messages based on how close to the maximum value the current value is.

The logic of "how close" is hard coded in limit-tracker.

## Example Usage

It could be used to keep track of a user's quota for the number
of API calls they're allowed to make, for example.
The following snippet code is from [src/main.rs](./src/main.rs).
Each time `reward()` is called, it will track the counter
and check whether it has exceed some limit. If it is the case,
just send corresponding message. This rule is coded inside
`set_value()` method in `limit-tracker`.

```rust
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

    a.reward(); // send --> Warning: You've used up over 75% of your quota!

    a.reward(); // send --> warning: You've used up over 90% of your quota!
    a.reward(); // send --> Error: You are over your quota!
}
```

## Design

The library will only provide the functionality of
tracking how close to the maximum a value is and what the messages should be at what times.

Applications that use the library will be expected to provide
the mechanism for sending the messages: the application could
put a message in the application, send an email, send a text message,
or something else.
The library doesn't need to know that detail. All it needs is something
that implements a trait we've provided -- `Messenger`. 

dive into the code [src/lib.rs](./src/lib.rs)

```rust
pub fn new(messenger: &'a T, max: usize) -> Self {
    LimitTracker {
        messenger,
        value: 0,
        max,
    }
}

// rules to track how close a value is to a maximum value
// and warn when the value is at certain levels
pub fn set_value(&mut self, value: usize) {
    // --snippet--

    if percentage_of_max >= 0.75 {
        self.messenger.send(WARNING);
    }

    // --snippet--
}
```

## Testing via Mock Objects

We want to test the behavior of the `set_value` method on `LimitTracker`.
Just use a mock object `MockMessenger` in place of the actual `Messenger`.

We can change what we pass in for the value parameter, but `set_value`
doesn't return anything for us to make assertions on.
We can use either of the following ways instead:

* `MockMessenger` send message to some place and check it
* `MockMessenger` store the message inside and then check it

Our test case makes use of the 2nd way:

```rust
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
```

Since the `Messenger` trait has one method called `send` that takes
an ***immutable*** reference to self and the text of the message.
This trait is the interface our ***mock object*** needs to implement
so that the mock can be used in the same way a real object is.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}
```

This is a situation in which ***interior mutability*** can help!
We'll store the `sent_messages` within a `RefCell<T>`, and then
the send method will be able to modify `sent_messages` to store
the messages we've seen.

We can now make assertions on it.
