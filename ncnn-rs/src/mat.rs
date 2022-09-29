use crate::allocator::Allocator as ncnn_Allocator;
use core::fmt;
use ncnn_bind::*;
use std::os::raw::c_void;

pub struct Mat {
    ptr: ncnn_mat_t,
}

impl Mat {
    pub fn new() -> Mat {
        let ptr = unsafe { ncnn_mat_create() };
        Mat { ptr }
    }

    pub fn create_1d(w: i32, alloc: &ncnn_Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_1d(w, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_2d(w: i32, h: i32, alloc: &ncnn_Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_2d(w, h, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_3d(w: i32, h: i32, c: i32, alloc: &ncnn_Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_3d(w, h, c, alloc.get()) };
        Mat { ptr }
    }

    // same as OpenCV Mat API https://docs.rs/opencv/latest/opencv/core/struct.Mat.html
    pub fn create_external_1d(w: i32, data: *mut c_void, alloc: &ncnn_Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_external_1d(w, data, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_external_2d(w: i32, h: i32, data: *mut c_void, alloc: &ncnn_Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_external_2d(w, h, data, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_external_3d(
        w: i32,
        h: i32,
        c: i32,
        data: *mut c_void,
        alloc: &ncnn_Allocator,
    ) -> Mat {
        let ptr = unsafe { ncnn_mat_create_external_3d(w, h, c, data, alloc.get()) };
        Mat { ptr }
    }

    // setter
    pub fn fill(&mut self, value: f32) {
        unsafe { ncnn_mat_fill_float(self.ptr, value) };
    }

    // getter
    pub fn get_dims(&self) -> i32 {
        unsafe { ncnn_mat_get_dims(self.ptr) }
    }

    pub fn get_w(&self) -> i32 {
        unsafe { ncnn_mat_get_w(self.ptr) }
    }
    pub fn get_h(&self) -> i32 {
        unsafe { ncnn_mat_get_h(self.ptr) }
    }
    pub fn get_c(&self) -> i32 {
        unsafe { ncnn_mat_get_c(self.ptr) }
    }

    pub fn get_elemsize(&self) -> u64 {
        (unsafe { ncnn_mat_get_elemsize(self.ptr) }) as u64
    }
    pub fn get_elempack(&self) -> i32 {
        unsafe { ncnn_mat_get_elempack(self.ptr) }
    }
    pub fn get_cstep(&self) -> u64 {
        unsafe { ncnn_mat_get_cstep(self.ptr) }
    }
    pub fn get_data(&self) -> *mut ::std::os::raw::c_void {
        unsafe { ncnn_mat_get_data(self.ptr) }
    }

    pub fn ptr(&self) -> ncnn_mat_t {
        self.ptr
    }

    pub fn mut_ptr(&mut self) -> *mut ncnn_mat_t {
        &mut self.ptr
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mat")
            .field("dims", &self.get_dims())
            .field("c", &self.get_c())
            .field("h", &self.get_h())
            .field("w", &self.get_w())
            .field("elemsize", &self.get_elemsize())
            .finish()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            ncnn_mat_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_getter_and_setter() {
        use crate::mat::*;
        let alloc = Allocator::new();
        let m: Mat = Mat::create_3d(224, 224, 3, alloc);
        assert_eq!(224, m.get_h());
        assert_eq!(224, m.get_w());
        assert_eq!(3, m.get_c());
    }
}
