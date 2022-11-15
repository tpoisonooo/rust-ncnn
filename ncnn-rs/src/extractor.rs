use ncnn_bind::*;
use std::{ffi::CString, marker::PhantomData};

pub struct Extractor<'a> {
    ptr: ncnn_extractor_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Extractor<'a> {
    pub(crate) fn from_ptr(ptr: ncnn_extractor_t) -> Self {
        Self {
            ptr,
            _phantom: PhantomData::default(),
        }
    }

    /// Sets extractor option.
    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe { ncnn_extractor_set_option(self.ptr, opt.ptr()) };
    }

    /// Sets input tensor by a given name.
    pub fn input(&mut self, name: &str, mat: &'a crate::mat::Mat) -> anyhow::Result<()> {
        let c_str = CString::new(name).unwrap();
        if unsafe { ncnn_extractor_input(self.ptr, c_str.as_ptr(), mat.ptr()) } != 0 {
            anyhow::bail!("Error setting input for layer `{}`", name);
        } else {
            Ok(())
        }
    }

    /// Runs network inferrence and returns output tensor by a given name.
    pub fn extract(self, name: &str, mat: &mut crate::mat::Mat) -> anyhow::Result<()> {
        let c_str = CString::new(name).unwrap();
        if unsafe { ncnn_extractor_extract(self.ptr, c_str.as_ptr(), mat.mut_ptr()) } != 0 {
            anyhow::bail!("Error running extract on layer `{}`", name);
        } else {
            Ok(())
        }
    }
}

impl<'a> Drop for Extractor<'a> {
    fn drop(&mut self) {
        unsafe {
            ncnn_extractor_destroy(self.ptr);
        }
    }
}
