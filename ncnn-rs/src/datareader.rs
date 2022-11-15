use libc::memset;
use ncnn_bind::*;

pub type ScanFn = unsafe extern "C" fn(
    dr: ncnn_datareader_t,
    format: *const ::std::os::raw::c_char,
    p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int;

pub type ReadFn = unsafe extern "C" fn(
    dr: ncnn_datareader_t,
    buf: *mut ::std::os::raw::c_void,
    size: size_t,
) -> size_t;

unsafe extern "C" fn empty_scan(
    _dr: ncnn_datareader_t,
    _format: *const ::std::os::raw::c_char,
    _p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    0
}

unsafe extern "C" fn empty_read(
    _dr: ncnn_datareader_t,
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
    /// Creates an new [DataReader].
    ///
    /// # Safety
    ///
    /// Must not be used until scan and read functions are set.
    pub unsafe fn new() -> Self {
        Self {
            ptr: ncnn_datareader_create(),
        }
    }

    /// Creates an empty [DataReader] that always reads zero bytes.
    pub fn empty() -> Self {
        Self {
            ptr: unsafe {
                let ptr = ncnn_datareader_create();
                (*ptr).scan = Some(empty_scan);
                (*ptr).read = Some(empty_read);
                ptr
            },
        }
    }

    pub unsafe fn set_scan(&mut self, function_ptr: Option<ScanFn>) {
        (*(self.ptr)).scan = function_ptr;
    }

    pub unsafe fn set_read(&mut self, function_ptr: Option<ReadFn>) {
        (*(self.ptr)).read = function_ptr;
    }

    pub(crate) fn ptr(&self) -> ncnn_datareader_t {
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
    fn empty_datareader() {
        use crate::datareader::*;
        let _ = DataReader::empty();
    }
}
