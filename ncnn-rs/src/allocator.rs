use ncnn_sys::*;
use std::ptr::null_mut;

pub struct Allocator {
    ptr: ncnn_allocator_t,
}

impl Allocator {
    pub fn new() -> crate::allocator::Allocator {
        Allocator { ptr: null_mut() }
    }

    // pub(crate) fn new(unlock: bool) -> crate::allocator::Allocator {
    //     let ptr;
    //     unsafe {
    //         if !unlock {
    //             ptr = ncnn_allocator_create_pool_allocator();
    //         } else {
    //             ptr = ncnn_allocator_create_unlocked_pool_allocator();
    //         }
    //     }
    //     Allocator { ptr }
    // }

    pub fn get(&self) -> ncnn_allocator_t {
        self.ptr
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                ncnn_allocator_destroy(self.ptr);
            }
        }
    }
}
