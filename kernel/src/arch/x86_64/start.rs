use arch::*;
use multiboot2;
use ::kernel_main;

#[no_mangle]
pub extern "C" fn kernel_start(multiboot_address: usize) {
    let boot_info = unsafe { multiboot2::load(multiboot_address) };

    enable_nxe_bit();
    enable_write_protect_bit();

    vga::init();

    println!("Kernel started.");

    clock::init();
    memory::init(boot_info);
    interrupts::init();

    // TODO: Other initialization code here

    divide_by_zero();

    println!("It did not crash!");
    loop {}

    kernel_main();
}

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::{cr0, cr0_write, CR0_WRITE_PROTECT};

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}
