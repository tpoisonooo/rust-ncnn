mod allocator;
mod datareader;
mod extractor;
mod mat;
mod net;
mod option;

pub use allocator::*;
pub use datareader::*;
pub use extractor::*;
pub use mat::*;
pub use net::*;
pub use option::*;

pub use ncnn_bind as ffi;

use std::ffi::CStr;

pub fn version() -> &'static str {
    let c_buf = unsafe { ffi::ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}
