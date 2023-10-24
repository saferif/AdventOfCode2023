use core::alloc::{GlobalAlloc, Layout};
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

const ALLOC_ALIGNMENT: usize = 4;

#[global_allocator]
static A: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[no_mangle]
unsafe extern "C" fn alloc(size: usize) -> usize {
    A.alloc(Layout::from_size_align_unchecked(size, ALLOC_ALIGNMENT)) as usize
}

#[no_mangle]
unsafe extern "C" fn dealloc(ptr: usize, size: usize) {
    A.dealloc(
        ptr as *mut u8,
        Layout::from_size_align_unchecked(size, ALLOC_ALIGNMENT),
    )
}
