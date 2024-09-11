threadpool
===

`threadpool` is a library to maintain a pool of thread
for running tasks.

***Features:***

* fixed numbler of threads reside in memory
* all threads are running concurrently and independently
* tasks are running asynchronously, i.e., users do not need to
wait for it to complete before doing other jobs. 

***Mechnism:***

1. The user creates a `pool` of threads via `ThreadPool::new(size)`
and these threads reside in memory waiting for running tasks.

2. Each time the user wants to run a task, just throw it to
the pool via `pool.execute(task)`, and return
to other jobs.

3. The `pool` will take an idle thread to run the task.
Current underlying implementation:
This `execute()` method just sends the task to a FIFO queue,
any idle thread will pick up a task from queue and run it.
When the task is finished, the thread will return to the queue
and begin a new cycle.

API
---

```rust
pub fn new(size: usize) -> ThreadPool {/* do something */}

pub fn execute<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{/* do something */}
```

Example
---

**threadpool** can be used anywhere that needs such functionality.

The following example is a tcp server that handles multiple connections
concurrently.

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // at most 4 tasks are running at the same time
    let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let task = || handle_connection(stream);
        pool.execute(task); // asynchronously
    }
}
```

