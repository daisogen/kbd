// Very simple MPSC implementation, by ChatGPT with GPT-4 backend because I
// *really* don't care about this. I just want it to work.

use std::sync::{Condvar, Mutex};

pub struct Mpsc<T> {
    data: Mutex<Vec<T>>,
    condvar: Condvar,
}

impl<T> Mpsc<T> {
    pub fn new() -> Self {
        Mpsc {
            data: Mutex::new(Vec::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        data.push(value);
        self.condvar.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = self.condvar.wait(data).unwrap();
        }
        data.pop().unwrap()
    }
}
