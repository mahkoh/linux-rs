pub use linux_macros::{kstr_pub as kstr};

use crate::kty::{c_char};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct KStr(*const c_char);

impl KStr {
    pub unsafe fn new(ptr: *const c_char) -> KStr {
        KStr(ptr)
    }

    pub fn as_ptr(self) -> *const c_char {
        self.0
    }
}
