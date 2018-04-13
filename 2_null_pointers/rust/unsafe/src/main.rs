extern crate libc;

use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    let s = String::from("Hello, world!");
    let s2;
    let p = s.as_ptr();
    println!("Original: {}", s);
    println!("Pointer:  {:?}", p);
    let test = vec![1,2,3];
    let format = CString::new("printf:   %s\n").unwrap().into_raw();
    println!("{:?}", test);
    unsafe {
        libc::printf(format, p as *const c_char);
        s2 = String::from_raw_parts(p as *mut _, s.len() + 20, s.capacity() + 20);
    }
    println!("Unsafe:   {}", s);
    println!("Corrupt:  {}", s2);
}
