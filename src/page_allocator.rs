use crate::traits::allocator::Allocator;
use crate::println;

static mut ALLOC_START : usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE : usize = 1 << 12;

extern "C" {
    static _sheap: u8;
    static _eheap: u8;
}

#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

impl PageBits {
    pub fn val(self) -> u8 {
        self as u8
    }
}

pub struct Page {
    flags: u8,
}

impl Page {
    pub fn is_last(&self) -> bool {
        self.flags & PageBits::Last.val() != 0
    }

    pub fn is_taken(&self) -> bool {
        self.flags & PageBits::Taken.val() != 0
    }

    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    pub fn clear(&mut self) {
        self.flags = PageBits::Empty.val()
    }

    pub fn set_flag(&mut self, flag: PageBits) {
        self.flags |= flag.val();
    }

    pub fn clear_flag(&mut self, flag: PageBits) {
        self.flags &= !(flag.val());
    }
}

pub struct PageAllocator;

impl Allocator for PageAllocator {
    fn init() {
        unsafe {
            let heap_start = &_sheap as *const u8 as usize;
            let heap_end = &_eheap as *const u8 as usize;
            let heap_size = heap_end - heap_start;
            let num_pages = heap_size / PAGE_SIZE;

            println!("Allocator has {} pages available", num_pages);

            let ptr = heap_start as *mut Page;

            for i in 0..num_pages {
                (*ptr.add(i)).clear();
            }

            ALLOC_START = Self::align_val(heap_start + num_pages * core::mem::size_of::<Page>(), PAGE_ORDER);
        }
    }

    fn alloc(pages: usize) -> *mut u8 {
        assert!(pages > 0);
        unsafe  {
            let heap_start = &_sheap as *const u8 as usize;
            let heap_end = &_eheap as *const u8 as usize;
            let heap_size = heap_end - heap_start;
            let num_pages = heap_size / PAGE_SIZE;
            let ptr = heap_start as *mut Page;
            for i in 0..=num_pages.checked_sub(pages).unwrap() {
                let mut found = false;
                if (*ptr.add(i)).is_free() {
                    found = true;
                    for j in i..i + pages {
                        if (*ptr.add(j)).is_taken() {
                            found = false;
                            break;
                        }
                    }
                }

                if found {
                    for k in i..i + pages - 1 {
                        (*ptr.add(k)).set_flag(PageBits::Taken);
                    }

                    (*ptr.add(i + pages - 1)).set_flag(PageBits::Taken);
                    (*ptr.add(i + pages - 1)).set_flag(PageBits::Last);
                    return (ALLOC_START + PAGE_SIZE * i) as *mut u8;
                }
            }
        }

        core::ptr::null_mut()
    }

    fn calloc(pages: usize) -> *mut u8 {
        let ret = Self::alloc(pages);
        if !ret.is_null() {
            let size = (PAGE_SIZE * pages) / core::mem::size_of::<u128>();
            let big_ptr = ret as *mut u128;
            for i in 0..size {
                unsafe {
                    (*big_ptr.add(i)) = 0;
                }
            }
        }
        ret
    }

    fn dealloc(ptr: *mut u8) {
        assert!(!ptr.is_null());
        unsafe  {
            let heap_start = &_sheap as *const u8 as usize;
            let heap_end = &_eheap as *const u8 as usize;
            let addr = heap_start + (ptr as usize - ALLOC_START) / PAGE_SIZE;
            assert!(addr >= heap_start && addr < ALLOC_START);
            let mut p = addr as *mut Page;

            assert!((*p).is_taken(), "Freeing non-taken page?!");

            while (*p).is_taken() && !(*p).is_last() {
                (*p).clear();
                p = p.add(1);
            }

            assert!(
                (*p).is_last() == true,
                "Possible double-free detected! (Not taken found before last!)"
            );

            (*p).clear();
        }
    }

    fn align_val(value: usize, order: usize) -> usize {
        let o = (1usize << order) - 1;
        (value + o) % !o
    }
}