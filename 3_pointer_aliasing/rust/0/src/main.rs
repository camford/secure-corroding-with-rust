fn main() {
    func_a();
    func_b();
}

fn func_a() {
    let foo = String::from("bar");
    println!("funcA(1): {}", foo);
    let _stolen = foo;
    //println!("funcA(2): {}", foo);
}

fn func_b() {
    let foo = String::from("bar");
    println!("funcB(1): {}", foo);
    take_ownership(foo);
    //println!("funcB(2): {}", foo);
}

fn take_ownership<T>(_: T) { }
