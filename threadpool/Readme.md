threadpool
===

`threadpool` is a library to maintain a pool of thread
for running tasks.

***Features:***

* fixed numbler of threads reside in memory
* all threads are running concurrently and independently
* tasks are running asynchronously, i.e., users do not need to
wait for it to complete and do other jobs. 

***Mechnism:***

1. The user creates a pool of threads via `ThreadPool::new(size)`
and these threads reside in memory waiting for running tasks.

2. Each time the user wants to run a task, just throw it to
the pool via `ThreadPool::execute(task)`, and return
to other jobs.
This `execute()` method just sends the task to a FIFO queue,
any idle thread will pick up a task from queue and run it.
When the task is finished, the thread will return to the queue
and begin a new circle.

API
---

```rust
pub fn new(size: usize) -> ThreadPool {}

pub fn execute<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{}
```

Example
---

threadpool can be used anywhere that needs such functionality:

[src/main.rs](./src/main.rs) creates a web server that uses
threadpool to run tasks of each http connection.

