//! The signatures of the `c` functions we will need

use libc::{c_char, c_void, size_t};


extern "C" {
    pub fn l_get_language(tc_handle: *const c_void , text: *const c_char, text_size: size_t) -> *mut c_char;
    pub fn l_textcat_Init(path: *const c_char) -> *mut c_void;
    pub fn textcat_Done(tc_handle: *const c_void);
}