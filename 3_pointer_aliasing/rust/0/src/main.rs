fn main() {
    funcA();
    funcB();
}

fn funcA() {
    let foo = String::from("bar");
    println!("funcA(1): {}", foo);
    let stolen = foo;
    //println!("funcA(2): {}", foo);
}

fn funcB() {
    let foo = String::from("bar");
    println!("funcB(1): {}", foo);
    take_ownership(foo);
    //println!("funcB(2): {}", foo);
}

fn take_ownership<T>(_: T) { }
