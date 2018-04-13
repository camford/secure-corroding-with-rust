use std::sync::Arc;
use std::thread;

static NTHREADS: i32 = 10;

fn race() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let var = Arc::new(0);

    for _ in 0..NTHREADS {
        let myvar = var.clone();
        children.push(thread::spawn(move || {
            *myvar += 1;
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    if *var != NTHREADS {
        println!("DATA RACE");
    }
}

fn main() {
    loop {
        race();
    }
}
