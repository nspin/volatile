#![allow(missing_docs)]

use core::ptr;

#[cfg(feature = "unstable")]
use core::intrinsics;

pub trait Ops: Copy + Default {}

pub trait UnitaryOps<T>: Ops {
    unsafe fn read(src: *const T) -> T;
    unsafe fn write(dst: *mut T, src: T);
}

pub trait BulkOps<T>: Ops {
    unsafe fn memmove(dst: *mut T, src: *const T, count: usize);
    unsafe fn memcpy(dst: *mut T, src: *const T, count: usize);
    unsafe fn memset(dst: *mut T, val: u8, count: usize);
}

#[derive(Default, Copy, Clone)]
pub struct VolatileOps;

impl Ops for VolatileOps {}

impl<T> UnitaryOps<T> for VolatileOps {
    unsafe fn read(src: *const T) -> T {
        unsafe { ptr::read_volatile(src) }
    }

    unsafe fn write(dst: *mut T, src: T) {
        unsafe { ptr::write_volatile(dst, src) }
    }
}

#[cfg(feature = "unstable")]
impl<T> BulkOps<T> for VolatileOps {
    unsafe fn memmove(dst: *mut T, src: *const T, count: usize) {
        unsafe { intrinsics::volatile_copy_memory(dst, src, count) }
    }

    unsafe fn memcpy(dst: *mut T, src: *const T, count: usize) {
        unsafe { intrinsics::volatile_copy_nonoverlapping_memory(dst, src, count) }
    }

    unsafe fn memset(dst: *mut T, val: u8, count: usize) {
        unsafe { intrinsics::volatile_set_memory(dst, val, count) }
    }
}
