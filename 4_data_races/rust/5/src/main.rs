use std::thread;
use std::sync::{Arc, Mutex};

static NTHREADS: i32 = 10;

fn race() {
    // Make a vector to hold the children which are spawned.
    let var = Arc::new(Mutex::new(0));
    let mut children = vec![];

    for _ in 0..NTHREADS {
        // Take ownership of our own 'copy'
        let myvar = var.clone();
        // Spin up another thread
        children.push(thread::spawn(move || {
            *myvar.lock().unwrap() += 1;
        }));
    } 

    // Wait for the thread to finish. Returns a result.
    for child in children {
        let _ = child.join();
    }

    let n = *var.lock().unwrap();
    if n!= NTHREADS {
        println!("DATA RACE!");
    }
}

fn main() {
    loop {
        race();
    }
}
