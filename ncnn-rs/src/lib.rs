pub extern crate self as ncnn_rs;

pub mod allocator;
pub mod datareader;
pub mod mat;
pub mod net;
pub mod option;

pub use allocator::*;
pub use datareader::*;
pub use mat::*;
pub use ncnn_bind::*;
pub use net::*;
pub use option::*;
use std::ffi::CStr;

pub fn version() -> &'static str {
    let c_buf = unsafe { ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}
