//! ELF executables

use alloc::{String, Vec};

use core::str;
use self::strtab::Strtab;
use self::section::{Section};
use self::symbol::Symbol;

#[cfg(target_arch = "x86")]
pub use goblin::elf32::{header, program_header, section_header, reloc, sym};

#[cfg(target_arch = "x86_64")]
pub use goblin::elf64::{header, program_header, section_header, reloc, sym};

mod strtab;
mod section;
mod symbol;

/// An ELF executable
pub struct Elf<'a> {
    pub data: &'a [u8],
    header: &'a header::Header
}

pub enum Error {
    Relocate(RelocateError),
    UndefinedSymbol(String),
    Malformed(String)
}

impl From<RelocateError> for Error {
    fn from(e: RelocateError) -> Error {
        Error::Relocate(e)
    }
}

enum RelocateError {

}

impl<'a> Elf<'a> {
    /// Create a ELF executable from data
    pub fn from(data: &'a [u8]) -> Result<Elf<'a>, String> {
        if data.len() < header::SIZEOF_EHDR {
            Err(format!("Elf: Not enough data: {} < {}", data.len(), header::SIZEOF_EHDR))
        } else if &data[..header::SELFMAG] != header::ELFMAG {
            Err(format!("Elf: Invalid magic: {:?} != {:?}", &data[..4], header::ELFMAG))
        } else if data.get(header::EI_CLASS) != Some(&header::ELFCLASS) {
            Err(format!("Elf: Invalid architecture: {:?} != {:?}", data.get(header::EI_CLASS), header::ELFCLASS))
        } else {
            Ok(Elf {
                data: data,
                header: unsafe { &*(data.as_ptr() as usize as *const header::Header) }
            })
        }
    }

    pub fn segments(&'a self) -> ElfSegments<'a> {
        ElfSegments {
            elf: self,
            i: 0
        }
    }

    pub fn sections(&'a self) -> ElfSections<'a> {
        ElfSections {
            elf: self,
            i: 0
        }
    }

    pub fn shdr_relocs(&self) -> Vec<reloc::Reloc> {
        self.sections()
            .filter(|section| section.sh_type() == section_header::SHT_REL || section.sh_type() == section_header::SHT_RELA)
            .flat_map(|section| {
                if section.sh_type() == section_header::SHT_REL {
                    section.entries::<reloc::Rel>()
                        .map(|rel| reloc::Reloc::from(rel.clone()))
                        .collect::<Vec<reloc::Reloc>>()
                } else if section.sh_type() == section_header::SHT_RELA {
                    section.entries::<reloc::Rel>()
                        .map(|rela| reloc::Reloc::from(rela.clone()))
                        .collect::<Vec<reloc::Reloc>>()
                } else {
                    panic!("Unexpected section type: {}", section.sh_type());
                }
            })
            .collect()
    }

    pub fn symbol(&'a self, table: usize, index: usize) -> Option<Symbol<'a>> {
        if let Some(symtab) = self.section(table) {
            if let Some(sym) = symtab.entry::<sym::Sym>(index) {
                Some(Symbol::new(symtab, sym))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn segment(&self, index: usize) -> Option<&program_header::ProgramHeader> {
        if index < self.header.e_phnum as usize {
            Some(unsafe {
                &* ((
                        self.data.as_ptr() as usize
                        + self.header.e_phoff as usize
                        + index* self.header.e_phentsize as usize
                    ) as *const program_header::ProgramHeader)
            })
        } else {
            None
        }
    }

    pub fn section(&'a self, index: usize) -> Option<Section<'a>> {
        if index < self.header.e_shnum as usize {
            Some(Section::new(self, unsafe {
                &* ((
                        self.data.as_ptr() as usize
                        + self.header.e_shoff as usize
                        + index * self.header.e_shentsize as usize
                    ) as *const section_header::SectionHeader)
            }))
        } else {
            None
        }
    }

    /// Get the entry field of the header
    pub fn entry(&self) -> usize {
        self.header.e_entry as usize
    }

    pub fn strtab(&self) -> Strtab {
        for section in self.sections() {
            if section.sh_type() == section_header::SHT_STRTAB {
                return Strtab::new(&section, 0x0);
            }
        }

        return Strtab::default();
    }

    pub fn shdr_strtab(&self) -> Strtab {
        let strtab_idx = self.header.e_shstrndx as usize;

        if let Some(section) = self.section(strtab_idx) {
            Strtab::new(&section, 0x0)
        } else {
            Strtab::default()
        }
    }

    pub fn load(&self) -> Result<(), Error> {
        try!(self.load_stage1());
        try!(self.load_stage2());
        Ok(())
    }

    fn load_stage1(&self) -> Result<(), Error> {
        for section in self.sections() {
            if section.sh_type() == section_header::SHT_NOBITS {
                if section.sh_size() == 0 {
                    continue;
                }

                if section.sh_flags() & section_header::SHF_ALLOC != 0 {
                    unimplemented!();
                    // Allocate and zero some memory
                    // void *mem = kmalloc(section.sectionHeader->size);
                    // memset(mem, 0, section.sectionHeader->size);

                    // Assign the memory offset to the section offset
                    // section.sectionHeader->offset = reinterpret_cast<uintptr_t>(mem) - baseAddress();

                    println!("Allocated memory for section: {}", section.sh_name());
                }
            }
        }

        Ok(())
    }

    fn load_stage2(&self) -> Result<(), Error> {
        for reloc in self.shdr_relocs() {
            unimplemented!();
            // try!(reloc.relocate());
        }

        Ok(())
    }

    // fn relocate(&self, section: &section_header::SectionHeader, reloc: &reloc::Reloc) -> Result<(), RelocateError> {
    //     let reference = self.section(section.sh_info).sh_offset + reloc->sh_offset;
    //     let mut symbol_value = 0;

    //     Symbol sym = symbol();
    //     if (!sym.isNull()) {
    //         symval = sym.value();
    //         if (symval == -1) {
    //             kerror() << "Unable to find symbol value!\n";
    //             return false;
    //         }
    //     }

    //     // Relocate based on type
    //     switch (type()) {
    //     case RelocationType::R_386_NONE:
    //         // No relocation
    //         return true;
    //     case RelocationType::R_386_32:
    //         // Symbol + Offset
    //         *ref = DO_386_32(symval, *ref);
    //         return true;
    //     case RelocationType::R_386_PC32:
    //         // Symbol + Offset - Section Offset
    //         *ref = DO_386_PC32(symval, *ref, (int) ref);
    //         return true;
    //     default:
    //         // Relocation type not supported, display error and return
    //         kerror() << "Unsupported Relocation Type: " << String::hex(static_cast<int>(type()))
    //                 << '\n';
    //         return false;
    //     }
    // }
}

pub struct ElfSegments<'a> {
    elf: &'a Elf<'a>,
    i: usize
}

impl<'a> Iterator for ElfSegments<'a> {
    type Item = &'a program_header::ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(segment) = self.elf.segment(self.i) {
            self.i += 1;
            Some(segment)
        } else {
            None
        }
    }
}

pub struct ElfSections<'a> {
    elf: &'a Elf<'a>,
    i: usize
}

impl<'a> Iterator for ElfSections<'a> {
    type Item = Section<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(section) = self.elf.section(self.i) {
            self.i += 1;
            Some(section)
        } else {
            None
        }
    }
}
