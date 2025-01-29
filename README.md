rust-apps
===

This repo intends to store rust apps implemented when I learn
the rust language.

Applications
---

* [guessing](./guessing) - an interactive guessing game
* [minigrep](./minigrep) - a command line tool to search string
from a text file: `minigrep string filename.txt`
* [blog_state](./blog_state) - a blog library that can manage a post.
* [blog](./blog) - a blog library similar to [blog_state](./blog_state).
* [threadpool](./threadpool) - a library to maintain a pool of thread
for running tasks concurrently, independently and asynchronously.
* [webscraper](./webscraper) - a command line tool of a web scraper.
* [list](./list) - a library that provides list-related utilities.

Related Technology
---

| Applications | Technologies | Language Basic| 
|:---:|:---|:---|
|[minigrep](./minigrep)| Dev Workflow, CLI, iterator filter |lifetime, [Result&lt;T, E&gt;][Result]|
|[blog_state](./blog_state)| (state) object-oriented, state transition,<br /> delegation | trait |
|[blog](./blog)| type-oriented, type-transformation| |
|[threadpool](./threadpool) | [CDD][CDD], concurrency, FIFO queue | thread, FnOnce, channel |
|[webscraper](./webscraper) | async | async |
|[list](./list) |API Design, generic|[Option&lt;T&gt;][Option], Iterator, mod, unsafe|

[CDD]: https://hzget.github.io/programming/basic/cdd.html
[Result]: https://doc.rust-lang.org/std/result/index.html
[Option]: https://doc.rust-lang.org/std/option/index.html
