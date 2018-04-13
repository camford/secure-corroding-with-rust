use std::thread;

static NTHREADS: i32 = 10;

// https://stackoverflow.com/questions/42849210/share-i32-mutably-between-threads
// https://ricardomartins.cc/2016/06/08/interior-mutability

fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let mut var = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            var.push(i);
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("{:#?}", var);
}
