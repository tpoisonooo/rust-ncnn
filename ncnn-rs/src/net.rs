use crate::datareader::DataReader;
use ncnn_bind::*;
use std::ffi::CString;
use std::marker::PhantomData;

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

    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe {
            ncnn_net_set_option(self.ptr, opt.ptr());
        }
    }

    pub fn load_param(&mut self, path: &str) -> i32 {
        let c_str = CString::new(path).unwrap();
        let ret = unsafe { ncnn_net_load_param(self.ptr, c_str.as_ptr()) };
        ret
    }

    pub fn load_model(&mut self, path: &str) -> i32 {
        let c_str = CString::new(path).unwrap();
        let ret = unsafe { ncnn_net_load_model(self.ptr, c_str.as_ptr()) };
        ret
    }

    pub fn load_model_datareader(&mut self, dr: &DataReader) -> i32 {
        unsafe { ncnn_net_load_model_datareader(self.ptr, dr.ptr()) }
    }

    pub fn create_extractor(&mut self) -> Extractor<'_> {
        let ptr;
        unsafe {
            ptr = ncnn_extractor_create(self.ptr);
        }
        Extractor::from_ptr(ptr)
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            ncnn_net_destroy(self.ptr);
        }
    }
}

pub struct Extractor<'a> {
    ptr: ncnn_extractor_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Extractor<'a> {
    fn from_ptr(ptr: ncnn_extractor_t) -> Self {
        Self {
            ptr,
            _phantom: PhantomData::default(),
        }
    }

    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe { ncnn_extractor_set_option(self.ptr, opt.ptr()) };
    }

    pub fn input(&mut self, name: &str, mat: &'a crate::mat::Mat) -> i32 {
        let c_str = CString::new(name).unwrap();
        let stat = unsafe { ncnn_extractor_input(self.ptr, c_str.as_ptr(), mat.ptr()) };
        stat
    }

    pub fn extract(self, name: &str, mat: &mut crate::mat::Mat) -> i32 {
        let c_str = CString::new(name).unwrap();
        let stat = unsafe { ncnn_extractor_extract(self.ptr, c_str.as_ptr(), mat.mut_ptr()) };
        stat
    }
}

impl<'a> Drop for Extractor<'a> {
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
