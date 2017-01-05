pub const MAX_PAGES: usize = 8388608; // 32 GB of physical memory
const PAGE_SIZE: usize = 0x1000;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}

impl Frame {
    pub fn for_address(address: PhysicalAddress) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    pub fn address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}
