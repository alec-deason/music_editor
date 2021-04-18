use std::ffi::{CString, CStr};
use verovio::*;

fn main() {
    let pointer = unsafe { vrvToolkit_constructorResourcePath(CString::new("/usr/local/share/verovio/").unwrap().as_ptr()) };
    let mut options = unsafe { vrvToolkit_getAvailableOptions(pointer) };
    println!("{:?}", unsafe { CStr::from_ptr(options) });
}
