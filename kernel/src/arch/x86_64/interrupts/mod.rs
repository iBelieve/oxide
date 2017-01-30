use spin::Once;

mod idt;
mod gdt;

macro_rules! save_scratch_registers {
    () => {
        asm!("push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11
        " :::: "intel", "volatile");
    }
}

macro_rules! restore_scratch_registers {
    () => {
        asm!("pop r11
              pop r10
              pop r9
              pop r8
              pop rdi
              pop rsi
              pop rdx
              pop rcx
              pop rax
            " :::: "intel", "volatile");
    }
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                save_scratch_registers!();
                asm!("mov rdi, rsp
                      add rdi, 9*8 // calculate exception stack frame pointer
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame))
                      : "rdi" : "intel", "volatile");
                restore_scratch_registers!();
                asm!("iretq"
                      :::: "intel", "volatile");
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
                save_scratch_registers!();
                asm!("mov rsi, [rsp + 9*8] // load error code into rsi
                      mov rdi, rsp
                      add rdi, 10*8 // calculate exception stack frame pointer
                      sub rsp, 8 // align the stack pointer
                      call $0
                      add rsp, 8 // undo stack pointer alignment
                      " :: "i"($name as extern "C" fn(&ExceptionStackFrame, u64))
                      : "rdi","rsi" : "intel");
                restore_scratch_registers!();
                asm!("add rsp, 8 // pop error code
                      iretq" :::: "intel", "volatile");
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

static GDT: Once<gdt::Gdt> = Once::new();

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(3, handler!(breakpoint_handler));
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

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n\n{:#?}",
             stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n\n{:#?}",
            stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) {
    use x86::shared::control_regs;
    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\
              \n    Error code: {:?}\
              \n\
              \n{:#?}",
            unsafe { control_regs::cr2() }, PageFaultErrorCode::from_bits(error_code).unwrap(),
            stack_frame);
    loop {}
}

extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n\n{:#?}",
            stack_frame.instruction_pointer, stack_frame);
}
