mod entry;
mod mapper;
mod table;
mod temp_page;

pub use self::entry::*;
pub use self::mapper::Mapper;

use arch::memory::{Frame, Page, PAGE_SIZE, VGA_BUFFER};
use arch::memory::frame_allocator::FrameAllocator;
use core::ops::{Deref, DerefMut};
use multiboot2::BootInformation;
use self::temp_page::TemporaryPage;
use super::KERNEL_OFFSET;

pub const MAX_FRAMES: usize = 8388608; // 32 GB of physical memory

const ENTRY_COUNT: usize = 512;

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F>(&mut self, table: &mut InactivePageTable,
                   temporary_page: &mut TemporaryPage, f: F)
            where F: FnOnce(&mut ActivePageTable) {
        use x86::shared::{control_regs, tlb};
        let flush_tlb = || unsafe { tlb::flush_all() };

        {
            let backup = Frame::containing_address(
                unsafe { control_regs::cr3() } as usize);

            // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive mapping
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);
            flush_tlb();

            // execute f in the new context
            f(self);

            // restore recursive mapping to original p4 table
            p4_table[511].set(backup, PRESENT | WRITABLE);
            flush_tlb();
        }

        temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86::shared::control_regs;

        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(
                unsafe { control_regs::cr3() } as usize
            ),
        };
        unsafe {
            control_regs::cr3_write(new_table.p4_frame.start_address());
        }
        old_table
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(),
                active_table);
            // now we are able to zero the table
            table.clear();
            // set up recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

pub fn init<A>(allocator: &mut A, boot_info: &BootInformation) -> ActivePageTable
    where A: FrameAllocator
{
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe },
        allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");

        for section in elf_sections_tag.sections() {
            if !section.is_allocated() {
                continue;
            }

            println!("Mapping section at address: {:#x}, size: {:#x}",
                section.addr, section.size);

            assert!(section.addr as usize % PAGE_SIZE == 0,
                    "Section needs to be page-aligned!");

            let flags = EntryFlags::from_elf_section_flags(section);

            let start_page = Page::containing_address(section.start_address());
            let end_page = Page::containing_address(section.end_address() - 1);
            for page in Page::range_inclusive(start_page, end_page) {
                let frame = Frame::containing_address(page.start_address() - KERNEL_OFFSET);
                // mapper.identity_map(frame.clone(), flags, allocator);
                mapper.map_to(page, frame, flags, allocator);
            }
        }

        // identity map the VGA text buffer
        let vga_buffer_frame = Frame::containing_address(VGA_BUFFER);
        mapper.identity_map(vga_buffer_frame, WRITABLE, allocator);

        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);

        // identity map the multiboot mudules list
        for module in boot_info.module_tags() {
            let module_start = Frame::containing_address(module.start_address() as usize);
            let module_end = Frame::containing_address(module.end_address() as usize - 1);
            println!("Mapping module {} from: {:#x} to: {:#x}" ,
                module.name(), module.start_address(), module.end_address());
            for frame in Frame::range_inclusive(module_start, module_end) {
                if frame < multiboot_start || frame > multiboot_end {
                    mapper.identity_map(frame, PRESENT, allocator);
                }
            }
        }

        // identity map the multiboot info structure
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, PRESENT, allocator);
        }
    });
    println!(" - Remapped the kernel");

    let old_table = active_table.switch(new_table);

    // turn the old p4 page into a guard page
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address() + KERNEL_OFFSET);
    active_table.unmap(old_p4_page, allocator);
    println!(" - Guard page at {:#x}", old_p4_page.start_address());

    active_table
}
