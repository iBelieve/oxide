use core::ops::Add;
use multiboot2::BootInformation;
use self::frame_allocator::AreaFrameAllocator;

pub use self::stack_allocator::Stack;

pub mod heap;
pub mod paging;
pub mod frame_allocator;
pub mod stack_allocator;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub const PML4_SIZE: usize = 0x0000_0080_0000_0000;
pub const KERNEL_SIZE: usize = PML4_SIZE;

/// Offset of recursive paging
pub const RECURSIVE_PAGE_OFFSET: usize = (-(PML4_SIZE as isize)) as usize;

/// Offset of kernel
pub const KERNEL_OFFSET: usize = RECURSIVE_PAGE_OFFSET - KERNEL_SIZE;

pub const KERNEL_HEAP_START: usize = KERNEL_OFFSET + KERNEL_SIZE/2;
pub const KERNEL_HEAP_SIZE: usize = 128 * 1024 * 1024; // 128 MB

pub const VGA_BUFFER: usize = 0xb8000;
pub const PAGE_SIZE: usize = 0x1000;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}

impl Frame {
    pub fn containing_address(address: PhysicalAddress) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
 }

// Like Frame, but for virtual instead of physical addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: usize
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
                "Invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> VirtualAddress {
        self.number * PAGE_SIZE
    }

    fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end,
        }
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page { number: self.number + rhs }
    }
}

#[derive(Clone)]
pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start.clone();
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: frame_allocator::AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController { ref mut active_table,
                                    ref mut frame_allocator,
                                    ref mut stack_allocator } = self;
        stack_allocator.alloc_stack(active_table, frame_allocator,
                                    size_in_pages)
    }
}

pub fn init(boot_info: &BootInformation) -> MemoryController {
    assert_has_not_been_called!("memory::init must be called only once");

    let memory_map_tag = boot_info.memory_map_tag().expect(
        "Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect(
        "Elf sections tag required");

    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap() as usize;
    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max()
        .unwrap() as usize;
    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();

    println!("Kernel start: {:#x}, kernel end: {:#x}",
             kernel_start,
             kernel_end);
    println!("Multiboot start: {:#x}, multiboot end: {:#x}",
             multiboot_start,
             multiboot_end);

    let mut frame_allocator = AreaFrameAllocator::new(
            kernel_start as usize, kernel_end as usize, multiboot_start,
            multiboot_end, memory_map_tag.memory_areas());

    let mut active_page_table = paging::init(&mut frame_allocator, boot_info);

    let heap_end_page = heap::init(&mut active_page_table,
                                   &mut frame_allocator);

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start,
                                                      stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    println!("Memory manager initialized.");

    MemoryController {
        active_table: active_page_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}
