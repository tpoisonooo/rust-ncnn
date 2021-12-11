extern crate self as ncnn_rs;

pub mod allocator;
pub mod mat;
pub mod net;
pub mod option;

use ncnn_sys::*;
use std::ffi::CStr;

pub fn version() -> &'static str {
    let c_buf = unsafe { ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}
