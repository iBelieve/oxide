pub mod paging;
pub mod pmm;

pub use self::paging::{PhysicalAddress, VirtualAddress};
pub use self::paging::test_paging;

use multiboot2::BootInformation;

pub const VGA_BUFFER: usize = 0xb8000;


pub fn init(boot_info: &BootInformation, kernel_end: PhysicalAddress) {
    pmm::init(boot_info, kernel_end);

    println!("Memory manager initialized.");
}
