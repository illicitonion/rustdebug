use std::ffi::CStr;
use std::os::raw;

fn main() {
    let s = unsafe {
        let ptr: *const raw::c_char = 0 as *const raw::c_char;
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    };
    println!("{:?}", s);
}
