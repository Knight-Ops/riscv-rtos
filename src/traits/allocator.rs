pub trait Allocator {
    fn init();

    fn alloc(count: usize) -> *mut u8;

    fn calloc(count: usize) -> *mut u8;

    fn dealloc(ptr: *mut u8);

    fn align_val(value: usize, order: usize) -> usize;
}