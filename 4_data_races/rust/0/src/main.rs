use std::thread;

static NTHREADS: i32 = 10;

fn race() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let mut var = 0;

    for _ in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            var += 1;
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    if var != NTHREADS {
        println!("THAT'S WEIRD! {}", var);
    }
}

fn main() {
    loop {
        race();
    }
}
