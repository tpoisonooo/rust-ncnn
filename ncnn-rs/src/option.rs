use ncnn_sys::*;
use std::os::raw::c_int;

pub struct Option {
    ptr: ncnn_option_t,
}

impl Option {
    pub(crate) fn new() -> Option {
        let ptr;
        unsafe {
            ptr = ncnn_option_create();
        }
        Option {ptr}
    }

    pub fn set_num_threads(&self, num_threads: u32) {
        unsafe {
            ncnn_option_set_num_threads(self.ptr, num_threads as c_int);
        }
    }

    pub fn get_num_threads(&self, num_threads: u32) -> u32 {
        let num = unsafe {ncnn_option_get_num_threads(self.ptr)};
        num as u32
    }
}

impl Drop for Option {
    fn drop(&mut self) {
        unsafe {
            ncnn_option_destroy(self.ptr);
        }
    }
}

