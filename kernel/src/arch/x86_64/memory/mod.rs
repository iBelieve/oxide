pub mod heap;
pub mod paging;
pub mod pmm;

use core::ops::DerefMut;
use multiboot2::BootInformation;

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

struct PageIter {
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

pub fn init(boot_info: &BootInformation) {
    assert_has_not_been_called!("memory::init must be called only once");

    // let memory_map_tag = boot_info.memory_map_tag().expect(
    //     "Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect(
        "Elf sections tag required");

    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap() as usize;
    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max()
        .unwrap() as usize;

    println!("Kernel start: {:#x}, kernel end: {:#x}",
             kernel_start,
             kernel_end);
    println!("Multiboot start: {:#x}, multiboot end: {:#x}",
             boot_info.start_address(),
             boot_info.end_address());

    pmm::init(boot_info, kernel_end - KERNEL_OFFSET);

    let mut active_page_table = paging::init(pmm::ALLOCATOR.lock().deref_mut(), boot_info);

    heap::init(&mut active_page_table, pmm::ALLOCATOR.lock().deref_mut());

    println!("Memory manager initialized.");
}
