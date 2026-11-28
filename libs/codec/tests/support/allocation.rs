use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
struct CountingAllocator;

#[derive(Clone, Copy, Debug, Default)]
pub struct AllocationCount {
    pub allocations: usize,
    pub reallocations: usize,
    pub requested_bytes: usize,
}

thread_local! {
    static ALLOCATION_COUNT: Cell<Option<AllocationCount>> = const { Cell::new(None) };
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn record_allocation(bytes: usize, reallocation: bool) {
    let _ = ALLOCATION_COUNT.try_with(|count| {
        if let Some(mut value) = count.get() {
            if reallocation {
                value.reallocations += 1;
            } else {
                value.allocations += 1;
            }
            value.requested_bytes += bytes;
            count.set(Some(value));
        }
    });
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size(), false);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size(), false);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(pointer, layout);
        }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() {
            record_allocation(size, true);
        }
        pointer
    }
}

pub struct AllocationMeasurement;

impl AllocationMeasurement {
    pub fn start() -> Self {
        ALLOCATION_COUNT.with(|count| {
            assert!(count.get().is_none());
            count.set(Some(AllocationCount::default()));
        });
        Self
    }

    pub fn snapshot(&self) -> AllocationCount {
        ALLOCATION_COUNT.with(|count| count.get().unwrap())
    }
}

impl Drop for AllocationMeasurement {
    fn drop(&mut self) {
        ALLOCATION_COUNT.with(|count| count.set(None));
    }
}
