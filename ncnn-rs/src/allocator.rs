use ncnn_bind::*;

pub struct Allocator {
    ptr: ncnn_allocator_t,
}

impl Allocator {
    /// Creates a new pool allocator.
    ///
    /// # Safety
    ///
    /// Using the allocator when creating matrix results in a segmentation fault.
    pub unsafe fn new() -> crate::allocator::Allocator {
        Allocator {
            ptr: ncnn_allocator_create_pool_allocator(),
        }
    }

    /// Creates a new unlocked pool allocator.
    ///
    /// # Safety
    ///
    /// Using the allocator when creating matrix results in a segmentation fault.
    pub unsafe fn new_unlocked() -> crate::allocator::Allocator {
        Allocator {
            ptr: ncnn_allocator_create_unlocked_pool_allocator(),
        }
    }

    pub(crate) fn ptr(&self) -> ncnn_allocator_t {
        self.ptr
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            ncnn_allocator_destroy(self.ptr);
        }
    }
}
