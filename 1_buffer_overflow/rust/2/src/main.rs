use std::env;
use std::str::FromStr;

fn main() {
    let buf: [i32; 5] = [1, 2, 3, 4, 5];
    println!("buf[4]: {}", buf[4]);
    if let Some(arg) = env::args().nth(1) {
        if let Ok(i) = usize::from_str(&arg) {
            println!("buf[{}]: {}", i, buf[i]);
        }
    }
}
