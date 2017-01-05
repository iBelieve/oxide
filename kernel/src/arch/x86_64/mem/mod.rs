pub const VGA_BUFFER: usize = 0xb8000;

pub mod paging;
pub mod pmm;

pub use self::paging::{PhysicalAddress, VirtualAddress};

pub fn init(kernel_end: PhysicalAddress) {
    pmm::init(kernel_end);

    println!("Memory manager initialized.");
}
