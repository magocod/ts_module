use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Fork;

pub struct Philosopher {
    pub name: String,
    // ANCHOR_END: Philosopher
    pub left_fork: Arc<Mutex<Fork>>,
    pub right_fork: Arc<Mutex<Fork>>,
    pub thoughts: mpsc::SyncSender<String>,
}

// ANCHOR: Philosopher-think
impl Philosopher {
    pub fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }
    // ANCHOR_END: Philosopher-think

    // ANCHOR: Philosopher-eat
    pub fn eat(&self) {
        // ANCHOR_END: Philosopher-eat
        println!("{} is trying to eat", &self.name);
        let _ = self.left_fork.lock().unwrap();
        let _ = self.right_fork.lock().unwrap();

        // ANCHOR: Philosopher-eat-end
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}
