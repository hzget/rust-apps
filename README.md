rust-apps
===

This repo intends to store rust apps implemented when I learn
the rust language.

Applications
---

* [minigrep](./minigrep) - a simple command line tool to search string
from a text file. It also shows the workflow of development using rust.
* [blog_state](./blog_state) - a blog library that can manage a post.
It is implemented by ***state pattern*** programming.
* [blog](./blog) - a blog library similar to [blog_state](./blog_state).
But it is implemented using the ***type system*** of rust instead.
* [threadpool](./threadpool) - a library to maintain a pool of thread
for running tasks concurrently, independently and asynchronously.
* [list](./list) - a library that provides list-related utilities.

Related Technology
---

| Applications | Technologies |
|:---:|:---|
|[minigrep](./minigrep)| Dev Workflow, lifetime, iterator filter |
|[blog_state](./blog_state)| (state) object-oriented, state transition, delegation, trait |
|[blog](./blog)| type-oriented, type-transformation|
|[threadpool](./threadpool) | [CDD][CDD], multi-threads, FnOnce, channel, mpmc |
|[list](./list) | generic, api design |

[CDD]: https://hzget.github.io/programming/basic/cdd.html
