use spin::Once;

mod idt;
mod gdt;

static GDT: Once<gdt::Gdt> = Once::new();

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(
                          &ExceptionStackFrame) -> !)
                      : "rdi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("pop rsi // pop error code into rsi
                      mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(
                          &ExceptionStackFrame, u64) -> !)
                      : "rdi","rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

bitflags! {
    flags PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0,
        const CAUSED_BY_WRITE = 1 << 1,
        const USER_MODE = 1 << 2,
        const MALFORMED_TABLE = 1 << 3,
        const INSTRUCTION_FETCH = 1 << 4,
    }
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));

        idt
    };
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub fn init() {
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        gdt
    });
    gdt.load();

    IDT.load();

    println!("Interrupts initialized");
}

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n\n{:#?}",
             stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n\n{:#?}",
             stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame,
                                 error_code: u64) -> !
{
    use x86::shared::control_regs;
    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\
              \n    Error code: {:?}\
              \n\
              \n{:#?}",
             unsafe { control_regs::cr2() }, PageFaultErrorCode::from_bits(error_code).unwrap(),
             stack_frame);
    loop {}
}
