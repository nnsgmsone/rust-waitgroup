//! Golang like WaitGroup

use std::sync::{Arc, Condvar, Mutex};

/// A WaitGroup waits for a collection of coroutines to finish.
///
/// # Examples
/// ```
/// use rust_waitgroup::WaitGroup;
/// use std::thread;
///
/// let wg = WaitGroup::default();
/// let n = 10;
/// for _ in 0..n {
///    let wg = wg.clone();
///    wg.add(1);
///    thread::spawn(move || {
///         // do some work
///         wg.done();
///    });
/// }
/// wg.wait();
/// ```
#[derive(Clone)]
pub struct WaitGroup {
    counter: Arc<(Mutex<i64>, Condvar)>,
}

impl WaitGroup {
    pub fn new() -> Self {
        WaitGroup {
            counter: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }
    /// add adds delta, which may be negative, to the WaitGroup counter.
    /// if the counter becomes zero, all coroutines blocked on Wait are released.
    /// if the counter goes negative, add panics
    pub fn add(&self, delta: i64) {
        let (lock, cvar) = &*self.counter;
        let mut count = lock.lock().unwrap();
        *count += delta;
        if *count < 0 {
            panic!("negative WaitGroup counter");
        }
        if *count == 0 {
            cvar.notify_all();
        }
    }
    /// done decrements the WaitGroup counter by one.
    pub fn done(&self) {
        self.add(-1);
    }
    /// wait blocks until the WaitGroup counter is zero.
    pub fn wait(&self) {
        let (lock, cvar) = &*self.counter;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        WaitGroup::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::thread;

    #[test]
    fn it_works() {
        let count = Arc::new(AtomicI64::new(0));
        let wg = WaitGroup::default();
        let n = 10;
        for _ in 0..n {
            let wg = wg.clone();
            wg.add(1);
            // count += 1
            let count = count.clone();
            count.fetch_add(1, Ordering::Relaxed);
            thread::spawn(move || {
                // count -= 1
                count.fetch_sub(1, Ordering::Relaxed);
                wg.done();
            });
        }
        wg.wait();
        // assert count == 0
        assert_eq!(count.load(Ordering::Relaxed), 0);
    }
}
