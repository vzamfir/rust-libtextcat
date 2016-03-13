//! Rust wrapper for the libtextcat library.
//! It detects the language of an input text.

extern crate libc;

use libc::{c_char, c_void, size_t};
use std::ffi::{CString, CStr};
use std::str;

/// Basic libtextcat structure, containing the handle from libtextcat initialization.
pub struct Textcat{
	handle : *mut c_void,
}

impl Textcat{
	/// Contructor for a new Textcat. It creates the libtextcat handle.
	pub fn new() -> Textcat {
		Textcat{
			handle: unsafe { l_textcat_Init(CString::new("libtextcat-2.2/langclass/LM/").unwrap().as_ptr()) },
		}
	}

	/// Detects the language / languages of the input text. Returns an array of &str containing all possible languages.
	pub fn get_language(&self, text : &str) -> Vec<&str> {
		let text_size = text.len();
		let c_buf: *mut c_char = unsafe { l_get_language(self.handle, CString::new(text).unwrap().as_ptr(), text_size as u64) };
		// convert the c_char result from the c funtion l_get_language to a rust &str.
		let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
		let buf: &[u8] = c_str.to_bytes();
		let languages_str: &str = str::from_utf8(buf).unwrap();
		// split the &str result into multiple &str representing the possible languages (without the characters '[' and ']') 
		// and put them into a vector.
		let languages: Vec<&str> = languages_str.split(|c| c== '[' || c == ']').filter(|x| !x.is_empty()).collect();

		return languages;
	}
}

impl Drop for Textcat {
	fn drop(&mut self) {
		unsafe { textcat_Done(self.handle) };		// free memory used by libtextcat handle.
	}
}

		
/// C signatures
extern "C" {
    pub fn l_get_language(tc_handle: *const c_void , text: *const c_char, text_size: size_t) -> *mut c_char;
    pub fn l_textcat_Init(path: *const c_char) -> *mut c_void;
    pub fn textcat_Done(tc_handle: *const c_void);
}
