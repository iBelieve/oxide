use super::paging::{self, ActivePageTable};
use super::{Page, KERNEL_HEAP_START, KERNEL_HEAP_SIZE};
use super::frame_allocator::FrameAllocator;
use alloc_kernel;

pub fn init<A>(active_table: &mut ActivePageTable, frame_allocator: &mut A) -> Page
        where A: FrameAllocator {
    assert_has_not_been_called!("heap::init must be called only once");

    let heap_start_page = Page::containing_address(KERNEL_HEAP_START);
    let heap_end_page = Page::containing_address(KERNEL_HEAP_START + KERNEL_HEAP_SIZE - 1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::PRESENT | paging::GLOBAL | paging::WRITABLE | paging::NO_EXECUTE, frame_allocator);
    }

    unsafe { alloc_kernel::init(KERNEL_HEAP_START, KERNEL_HEAP_SIZE) };

    heap_end_page
}
