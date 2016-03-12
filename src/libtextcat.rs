extern crate libc;

use libc::{c_char, c_void, size_t, c_int};
use std::ffi::{CString, CStr};
use std::str;

pub struct Textcat{
	handle : *mut c_void,
}

impl Textcat{
	pub fn new() -> Textcat {
		Textcat{
			handle: unsafe { l_textcat_Init(CString::new("libtextcat-2.2/langclass/LM/").unwrap().as_ptr()) },
		}
	}

	pub fn get_language(&self, text : &str) -> &str {
		let text_size = text.len();
		let c_buf: *const c_char = unsafe { l_get_language(self.handle, CString::new(text).unwrap().as_ptr(), text_size as u64) };
		let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
		let buf: &[u8] = c_str.to_bytes();
		let str_slice: &str = str::from_utf8(buf).unwrap();
		return str_slice;
	}
}

		
//C signatures
extern "C" {
    pub fn l_get_language(tc_handle: *const c_void , text: *const c_char, text_size: size_t) -> *mut c_char;
    pub fn l_textcat_Init(path: *const c_char) -> *mut c_void;
}
