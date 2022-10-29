use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    threads: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id));
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
