use ncnn_sys::*;
use std::os::raw::c_int;

pub struct DataReader {
    ptr: ncnn_datareader_t,
}

impl DataReader {
    pub(crate) fn new() -> DataReader {
        let ptr;
        unsafe {
            ptr = ncnn_datareader_create();
        }
        DataReader { ptr }
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
}
