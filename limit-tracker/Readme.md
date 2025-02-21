# limit-tracker

`limit-tracker` create a library that tracks a value against a maximum value
and sends messages based on how close to the maximum value the current value is.

It could be used to keep track of a user's quota for the number
of API calls they're allowed to make, for example.
Please refer to [src/main.rs](./src/main.rs) for such case.

The library will only provide the functionality of
tracking how close to the maximum a value is and what the messages should be at what times.

Applications that use the library will be expected to provide
the mechanism for sending the messages: the application could
put a message in the application, send an email, send a text message,
or something else.
The library doesn't need to know that detail. All it needs is something
that implements a trait we'll provide called `Messenger`. 

dive into the code [src/lib.rs](./src/lib.rs)

