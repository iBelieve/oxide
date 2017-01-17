pub mod paging;
pub mod pmm;

use core::ops::DerefMut;
use multiboot2::BootInformation;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

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
#[derive(Debug, Clone, Copy)]
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

pub fn init(boot_info: &BootInformation, kernel_end: VirtualAddress) {
    assert_has_not_been_called!("memory::init must be called only once");

    pmm::init(boot_info, kernel_end);
    paging::remap_kernel(pmm::ALLOCATOR.lock().deref_mut(), boot_info);

    println!("Memory manager initialized.");
}
