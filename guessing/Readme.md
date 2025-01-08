Guessing Game
===

It is a guessing game used as a first hands-on project
to learn the rust language basic.

It comes from [The Book: The Rust Programming Language][The Book]

dive into the code: [src/main.rs](src/main.rs)

Requirement
---

We'll implement a classic beginner programming problem: a guessing game.
Here's how it works:

the program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess. After a guess is entered,
the program will indicate whether the guess is too low or too high.
If the guess is correct, the game will print a congratulatory message and exit.

Just as follows:

```bash
# cargo run
Guess the number!
Please input your guess.
50
You guessed: 50
Too small!
Please input your guess.
75
You guessed: 75
You win!
#
```

Breakdown of the task
---

* Parsing Guess from User Input (using `std::io`)
* Generating a Secret Number (using `rand` crate)
* Comparing the Guess to the Secret Number (using `match` expression)
* Giving Result/Prompt to the User

Language Basics that are used
---

* `std::io` and `println!`
* enum: `Result` and `std::cmp::Ordering`
* the `match` expression which is made up of ***arms***
* control flow, such as `loop`, `break`, `continue`
* mutable/immutable variable, ***Shadowing***
* how to include an external crate
* cargo, ***Semantic Versioning***

[The Book]: https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html#programming-a-guessing-game
