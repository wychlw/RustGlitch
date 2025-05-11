#![feature(const_heap)]
#![feature(thin_box)]
use std::intrinsics;

const BAR: *mut i32 = unsafe { intrinsics::const_allocate(4, 4) as *mut i32 };

// https://github.com/rust-lang/rust/issues/140268