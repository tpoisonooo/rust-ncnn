use libc::memset;
use ncnn_sys::*;
use std::os::raw::c_int;

unsafe extern "C" fn default_scan(
    dr: ncnn_datareader_t,
    format: *const ::std::os::raw::c_char,
    p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    0
}

unsafe extern "C" fn default_read(
    dr: ncnn_datareader_t,
    buf: *mut ::std::os::raw::c_void,
    size: size_t,
) -> size_t {
    memset(buf, 0, size as usize);
    size
}

pub struct DataReader {
    ptr: ncnn_datareader_t,
}

impl DataReader {
    pub fn new() -> DataReader {
        let ptr;
        unsafe {
            ptr = ncnn_datareader_create();
        }
        DataReader { ptr }
    }

    pub fn use_empty_config(&self) {
        unsafe {
            (*(self.ptr)).scan = Some(default_scan);
            (*(self.ptr)).read = Some(default_read);
        }
    }

    pub fn set_scan(
        &self,
        function_ptr: std::option::Option<
            unsafe extern "C" fn(
                dr: ncnn_datareader_t,
                format: *const ::std::os::raw::c_char,
                p: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) {
        unsafe {
            (*(self.ptr)).scan = function_ptr;
        }
    }

    pub fn set_read(
        &self,
        function_ptr: std::option::Option<
            unsafe extern "C" fn(
                dr: ncnn_datareader_t,
                buf: *mut ::std::os::raw::c_void,
                size: size_t,
            ) -> size_t,
        >,
    ) {
        unsafe {
            (*(self.ptr)).read = function_ptr;
        }
    }

    pub fn get(&self) -> ncnn_datareader_t {
        self.ptr
    }
}

impl Drop for DataReader {
    fn drop(&mut self) {
        unsafe {
            ncnn_datareader_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_datareader() {
        use crate::datareader::*;
        let _ = DataReader::new();
    }

    // use crate::datareader::*;
    // use ncnn_sys::*;
    // unsafe extern "C" fn empty_scan(
    //     dr: ncnn_datareader_t,
    //     format: *const ::std::os::raw::c_char,
    //     p: *mut ::std::os::raw::c_void,
    // ) -> ::std::os::raw::c_int {
    //     0
    // }

    // unsafe extern "C" fn empty_read(
    //     dr: ncnn_datareader_t,
    //     buf: *mut ::std::os::raw::c_void,
    //     size: size_t,
    // ) -> size_t {
    //     size
    // }

    #[test]
    fn empty_datareader() {
        use crate::datareader::*;
        let reader = DataReader::new();
        reader.use_empty_config();
        // reader.set_read(Some(empty_read));
        // reader.set_scan(Some(empty_scan));
    }
}
