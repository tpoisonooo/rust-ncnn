use ncnn_sys::*;
use std::os::raw::c_int;

pub struct Option {
    ptr: ncnn_option_t,
}

impl Option {
    pub fn new() -> Option {
        let ptr;
        unsafe {
            ptr = ncnn_option_create();
        }
        Option { ptr }
    }

    pub fn set_num_threads(&self, num_threads: u32) {
        unsafe {
            ncnn_option_set_num_threads(self.ptr, num_threads as c_int);
        }
    }

    pub fn get_num_threads(&self) -> u32 {
        let num = unsafe { ncnn_option_get_num_threads(self.ptr) };
        num as u32
    }

    pub fn get(&self) -> ncnn_option_t {
        self.ptr
    }
}

impl Drop for Option {
    fn drop(&mut self) {
        unsafe {
            ncnn_option_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_cpu_info() {
        use crate::option::*;
        let opt = Option::new();
        opt.set_num_threads(4);
        assert_eq!(4, opt.get_num_threads());
    }
}
