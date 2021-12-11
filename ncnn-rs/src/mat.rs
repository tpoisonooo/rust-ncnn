use crate::allocator::Allocator;
use ncnn_sys::*;
use std::os::raw::c_void;

pub struct Mat {
    ptr: ncnn_mat_t,
}

impl Mat {
    pub fn create_1d(w: i32, alloc: Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_1d(w, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_2d(w: i32, h: i32, alloc: Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_2d(w, h, alloc.get()) };
        Mat { ptr }
    }

    pub fn create_3d(w: i32, h: i32, c: i32, alloc: Allocator) -> Mat {
        let ptr = unsafe { ncnn_mat_create_3d(w, h, c, alloc.get()) };
        Mat { ptr }
    }

    // same as OpenCV Mat API https://docs.rs/opencv/latest/opencv/core/struct.Mat.html
    pub unsafe fn create_external_1d(w: i32, data: *mut c_void, alloc: Allocator) -> Mat {
        let ptr = ncnn_mat_create_external_1d(w, data, alloc.get());
        Mat { ptr }
    }

    pub unsafe fn create_external_2d(w: i32, h: i32, data: *mut c_void, alloc: Allocator) -> Mat {
        let ptr = ncnn_mat_create_external_2d(w, h, data, alloc.get());
        Mat { ptr }
    }

    pub unsafe fn create_external_3d(
        w: i32,
        h: i32,
        c: i32,
        data: *mut c_void,
        alloc: Allocator,
    ) -> Mat {
        let ptr = ncnn_mat_create_external_3d(w, h, c, data, alloc.get());
        Mat { ptr }
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
    fn basic_getter_and_setter () {
        use crate::mat::*;
        let alloc = Allocator::new();
        let m: Mat = Mat::create_3d(224, 224, 3, alloc);
        // println!("mat shape: {} {} {} ", m.get_h(), m.get_w(), m.get_c());
    }
}