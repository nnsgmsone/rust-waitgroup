# rust-waitgroup

rust-waitgroup is a lightweight synchronization primitive for managing concurrency in Rust programs, inspired by the waitgroup from Go.

## Features

- **Synchronization**: Efficiently waits for a collection of threads (or tasks) to complete before proceeding.
- **Lightweight**: Minimal overhead with simple API for adding tasks and waiting on their completion.
- **Thread Safety**: Ensures thread safety for synchronization across multiple threads.

## Getting Started

```rust
use rust_waitgroup::WaitGroup;
use std::thread;

let wg = WaitGroup::default();
let n = 10;
for _ in 0..n {
    let wg = wg.clone();
    wg.add(1);
    thread::spawn(move || {
         // do some work
         wg.done();
    });
}
wg.wait();
```

## License

`rust-waitgroup` source code is available under the GPL [License](/LICENSE).
