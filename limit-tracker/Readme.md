# limit-tracker

`limit-tracker` create a library that tracks a value against a maximum value
and sends messages based on how close to the maximum value the current value is.

## Example Usage

It could be used to keep track of a user's quota for the number
of API calls they're allowed to make, for example.
The following snippet code is from [src/main.rs](./src/main.rs).
Each time `reward()` is called, it will track the counter
and check whether it has exceed some limit. If it is the case,
just send corresponding message.

```rust
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
that implements a trait we'll provide called `Messenger`. 

dive into the code [src/lib.rs](./src/lib.rs)

```rust
pub fn new(messenger: &'a T, max: usize) -> Self {
    LimitTracker {
        messenger,
        value: 0,
        max,
    }
}

pub fn set_value(&mut self, value: usize) {
    // rules to track how close a value is to a maximum value
    // and warn when the value is at certain levels
    // --snippet--
}
```

## Testing via Mock Objects

