fn main() {
    func_a();
    func_b();
    func_c();
}

// Copying
fn func_a() {
    let foo = String::from("bar");
    println!("func_a(1): {}", foo);
    take_ownership(foo.clone());
    println!("func_a(2): {}", foo);
}

fn func_b() {
    let foo = String::from("bar");
    println!("func_b(1): {}", foo);
    let qux = return_ownership(foo);
    println!("func_b(2): {}", qux);
}

fn func_c() {
    let foo = String::from("bar");
    println!("func_c(1): {}", foo);
    borrow_ownership(&foo);
    println!("func_c(2): {}", foo);
}

fn take_ownership<T>(_: T) { }
fn return_ownership<T>(baz: T) -> T { return baz; }
fn borrow_ownership<T>(_: &T) { }