mod functions;
mod types;
pub use self::functions::*;
pub use self::types::*;

use std::ffi::CStr;
use std::os::raw::c_char;

pub fn ptr_to_string(str_ptr: *const c_char) -> Option<String> {
    if !str_ptr.is_null() {
        match unsafe { CStr::from_ptr(str_ptr) }.to_str().ok() {
            Some(str_slice) => Some(str_slice.to_owned()),
            None => None,
        }
    } else {
        None
    }
}
