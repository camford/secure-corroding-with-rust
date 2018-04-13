use std::ptr;

fn main() {
    let p: *mut i32 = ptr::null_mut();
    
    // Pretend p came from somewhere and we incorrectly assume it's never null

    unsafe {
        *p = 1;
    }
}
