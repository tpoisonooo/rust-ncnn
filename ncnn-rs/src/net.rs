use crate::datareader::DataReader;
use ncnn_sys::*;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct Net {
    ptr: ncnn_net_t,
}

impl Net {
    pub fn new() -> Net {
        let ptr;
        unsafe {
            ptr = ncnn_net_create();
        }
        Net { ptr }
    }

    pub fn get(&self) -> ncnn_net_t {
        self.ptr
    }

    pub fn set_option(&self, opt: &crate::option::Option) {
        unsafe {
            ncnn_net_set_option(self.ptr, opt.get());
        }
    }

    pub fn load_param(&self, path: &str) -> i32 {
        let c_str = CString::new(path).unwrap();
        let c_ptr = c_str.as_ptr() as *const c_char;
        let ret = unsafe { ncnn_net_load_param(self.ptr, c_ptr) };
        ret
    }

    pub fn load_model(&self, path: &str) -> i32 {
        let c_str = CString::new(path).unwrap();
        let c_ptr = c_str.as_ptr() as *const c_char;
        let ret = unsafe { ncnn_net_load_model(self.ptr, c_ptr) };
        ret
    }

    pub fn load_model_datareader(&self, dr: &DataReader) -> i32 {
        unsafe { ncnn_net_load_model_datareader(self.ptr, dr.get()) }
    }

    pub fn create_extractor(&self) -> Extractor {
        let ptr;
        unsafe {
            ptr = ncnn_extractor_create(self.ptr);
        }
        Extractor { ptr }
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            ncnn_net_destroy(self.ptr);
        }
    }
}

pub struct Extractor {
    ptr: ncnn_extractor_t,
}

impl Extractor {
    pub fn set_option(&self, opt: &crate::option::Option) {
        unsafe { ncnn_extractor_set_option(self.ptr, opt.get()) };
    }

    pub fn input(&self, name: &str, mat: &crate::mat::Mat) -> i32 {
        let c_str = CString::new(name).unwrap();
        let c_ptr = c_str.as_ptr() as *const c_char;

        let stat = unsafe { ncnn_extractor_input(self.ptr, c_ptr, mat.get()) };
        stat
    }

    pub fn extract(&self, name: &str, mat: &crate::mat::Mat) -> i32 {
        let c_str = CString::new(name).unwrap();
        let c_ptr = c_str.as_ptr() as *const c_char;

        let stat = unsafe { ncnn_extractor_extract(self.ptr, c_ptr, &mut mat.get()) };
        stat
    }
}

impl Drop for Extractor {
    fn drop(&mut self) {
        unsafe {
            ncnn_extractor_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_not_exist_model() {
        use crate::net::*;
        let net = Net::new();
        let ret = net.load_param("not_exist.param");
        assert_ne!(ret, 0);
    }
}
