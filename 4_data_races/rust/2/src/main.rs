use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    let mut var = vec![];

    // Spin up another thread
    let child1 = thread::spawn(move || {
        var.push(1);
        println!("this is thread number {}", 1)
    });

    // Spin up another thread
    let child2 = thread::spawn(move || {
        var.push(2);
        println!("this is thread number {}", 2)
    });

    // Wait for the threads to finish.
    let _ = child1.join();
    let _ = child2.join();
    println!("{:#?}", var);
}
