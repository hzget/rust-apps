Minigrep
===

This repo just shows how to develop a command line tool using Rust.

It's a case coming from The Book
[The Rust Programming Language][the book].


Normal steps for development:

1. Evaluate the requirement
2. Brainstorm solutions
3. Design the system structure
4. Code and Test
5. Optimization
6. Documentation

Requirement
---

Make our own version of the classic command line search tool `grep`
(Globally search a Regular Expression and Print).
It takes as its arguments a string and a file path.
Then it reads the file, finds lines in that file that contain the
string argument, and prints those lines. Just like this:

```bash
minigrep searchstring example-filename.txt
```

Extra requirement:

* write result to stdout, and write error info to stderr
* print helper information if its argment is incorrect
* support both case-sensitive and case-insensitive search,
and it depends on whether environment variable `IGNORE_CASE` is set.

Design the system structure
---

Tasks for this functionality

* parse arguments
* read file
* search string from that file
* print result

```bash
minigrep
   |---src/
        |--- main.rs
        |--- lib.rs
```

[main.rs](./main.rs) is a binary crate to finish the function.
It will:

* "run" the grep function
* print helper information for incorrect input
* print error information

[lib.rs](./lib.rs) is a library crate to implement searching funtions.
It also contains test cases for those functions.

* data structure "Config" to hold "options"
* searching functions
* a `build()` function to parse args and construct the "Config" structure
* a `run()` function to read file, search the string, and print result
* test cases

TDD - Test Driven Development
---

Public API

```rust
pub fn Config::build(args: &[String]) -> Result<Config, &str> {}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {}
```

Underlying Functions

```rust
fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {}
fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {}
```

test cases:

```bash
running 4 tests
test tests::case_build ... ok
test tests::case_run ... ok
test tests::case_search ... ok
test tests::case_search_case_insensitive ... ok
```

***Note:***

`minigrep` uses Env variable `IGNORE_CASE` as a config option,
the user shall avoid parallel testing. Just use the following:

```bash
cargo test -- --test-threads=1
```

Test Covarage
---

Test coverage is a valuable metric for assessing the thoroughness
of automated tests. It helps ensure that the code is well-tested
and reliable, though it should be used in conjunction with other
quality metrics.
The goal is to achieve a balance between high coverage and meaningful
tests that effectively catch potential issues in the code.

In Rust, tools like cargo tarpaulin can be used to measure test coverage.

```bash
# cargo tarpaulin --out Html
// result:
2024-08-17T02:09:19.262876Z  INFO cargo_tarpaulin::report: Coverage Results:
|| Tested/Total Lines:
|| src\lib.rs: 37/37 +0.00%
||
100.00% coverage, 37/37 lines covered, +0.00% change in coverage
```

It also generates a file "tarpaulin-report.html" to show the coverage.

Documentation
---

Rust supports documentation comment which will generate docs
automatically via rust doc commands:

```bash
cargo doc --open
```

It will build the HTML for current crate’s documentation (as well a
the documentation for all of the crate’s dependencies) and open th
result in a web browser. 

[the book]: https://rust-book.cs.brown.edu/
