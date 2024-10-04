rust-apps
===

This repo intends to store rust apps implemented when I learn
the rust language.

Applications
---

* [minigrep](./minigrep) - a command line tool to search string
from a text file: `minigrep string filename.txt`
* [blog_state](./blog_state) - a blog library that can manage a post.
* [blog](./blog) - a blog library similar to [blog_state](./blog_state).
* [threadpool](./threadpool) - a library to maintain a pool of thread
for running tasks concurrently, independently and asynchronously.
* [list](./list) - a library that provides list-related utilities.

Related Technology
---

| Applications | Technologies | Language Basic| 
|:---:|:---|:---|
|[minigrep](./minigrep)| Dev Workflow, CLI, iterator filter |lifetime|
|[blog_state](./blog_state)| (state) object-oriented, state transition,<br /> delegation | trait |
|[blog](./blog)| type-oriented, type-transformation| |
|[threadpool](./threadpool) | [CDD][CDD], concurrency, FIFO queue | thread, FnOnce, channel |
|[list](./list) |API Design, generic|Option, Iterator, mod, unsafe|

[CDD]: https://hzget.github.io/programming/basic/cdd.html
