use std::ptr;

fn main() {
    let bad_p: *mut i32 = ptr::null_mut();
    
    // Pretend p came from somewhere
    // Now we want to wrap it up so it's safe to use

    let good_p = ptr::NonNull::new(bad_p);

    // Stuff happens

    match good_p {
        Some(p) => {
            let ptr = p.as_ptr();
            unsafe {
                *ptr = 1;
            }
        },
        None => println!("Not messing with that null pointer")
    }
}
