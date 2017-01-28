use spin::Once;

mod idt;
mod gdt;

static GDT: Once<gdt::Gdt> = Once::new();

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, divide_by_zero_handler);

        idt
    };
}

pub fn init() {
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        gdt
    });
    gdt.load();

    IDT.load();
}

extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}
