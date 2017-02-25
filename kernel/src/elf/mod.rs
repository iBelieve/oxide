//! ELF executables

use alloc::{String, Vec};

use core::str;
use self::strtab::Strtab;

#[cfg(target_arch = "x86")]
pub use goblin::elf32::{header, program_header, section_header, reloc};

#[cfg(target_arch = "x86_64")]
pub use goblin::elf64::{header, program_header, section_header, reloc};

mod strtab;

/// An ELF executable
pub struct Elf<'a> {
    pub data: &'a [u8],
    header: &'a header::Header
}

enum LoadError {
    RelocateError(RelocateError)
}

impl From<RelocateError> for LoadError {
    fn from(e: RelocateError) -> LoadError {
        LoadError::RelocateError(e)
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
            data: self.data,
            header: self.header,
            i: 0
        }
    }

    pub fn sections(&'a self) -> ElfSections<'a> {
        ElfSections {
            data: self.data,
            header: self.header,
            i: 0
        }
    }

    pub fn shdr_relocs(&self) -> Vec<reloc::Reloc> {
        let mut relocs = vec![];
        if self.header.e_type == header::ET_REL {
            for section in self.sections() {
                if section.sh_type == section_header::SHT_REL {
                    let iter = Relocs {
                        data: self.data,
                        header: section,
                        is_rela: false,
                        i: 0
                    };
                    relocs.extend(iter);
                }
                if section.sh_type == section_header::SHT_RELA {
                    let iter = Relocs {
                        data: self.data,
                        header: section,
                        is_rela: true,
                        i: 0
                    };
                    relocs.extend(iter);
                }
            }
        }
        relocs
    }

    pub fn segment(&self, index: usize) -> Option<&program_header::ProgramHeader> {
        self.segments().nth(index)
    }

    pub fn section(&self, index: usize) -> Option<&section_header::SectionHeader> {
        self.sections().nth(index)
    }

    /// Get the entry field of the header
    pub fn entry(&self) -> usize {
        self.header.e_entry as usize
    }

    pub fn strtab(&self) -> Strtab {
        for section in self.sections() {
            if section.sh_type == section_header::SHT_STRTAB {
                let start = section.sh_offset as usize;
                let end = (section.sh_offset + section.sh_size) as usize;
                return Strtab::from_raw(&self.data[start..end], 0x0);
            }
        }

        return Strtab::default();
    }

    pub fn shdr_strtab(&self) -> Strtab {
        let strtab_idx = self.header.e_shstrndx as usize;

        if let Some(section) = self.section(strtab_idx) {
            let start = section.sh_offset as usize;
            let end = (section.sh_offset + section.sh_size) as usize;
            Strtab::from_raw(&self.data[start..end], 0x0)
        } else {
            Strtab::default()
        }
    }

    pub fn load(&self) -> Result<(), LoadError> {
        try!(self.load_stage1());
        try!(self.load_stage2());
        Ok(())
    }

    fn load_stage1(&self) -> Result<(), LoadError> {
        for section in self.sections() {
            if section.sh_type == section_header::SHT_NOBITS {
                if section.sh_size == 0 {
                    continue;
                }

                if (section.sh_flags as u32) & section_header::SHF_ALLOC != 0 {
                    unimplemented!();
                    // Allocate and zero some memory
                    // void *mem = kmalloc(section.sectionHeader->size);
                    // memset(mem, 0, section.sectionHeader->size);

                    // Assign the memory offset to the section offset
                    // section.sectionHeader->offset = reinterpret_cast<uintptr_t>(mem) - baseAddress();

                    println!("Allocated memory for section: {}", self.shdr_strtab().get(section.sh_name as usize));
                }
            }
        }

        Ok(())
    }

    fn load_stage2(&self) -> Result<(), LoadError> {
        for reloc in self.shdr_relocs() {
            unimplemented!();
            // try!(reloc.relocate());
        }

        Ok(())
    }
}

pub struct ElfSegments<'a> {
    data: &'a [u8],
    header: &'a header::Header,
    i: usize
}

pub struct ElfSections<'a> {
    data: &'a [u8],
    header: &'a header::Header,
    i: usize
}

pub struct Relocs<'a> {
    data: &'a [u8],
    header: &'a section_header::SectionHeader,
    is_rela: bool,
    i: usize
}

impl<'a> Relocs<'a> {
    fn is_rela(&self) -> bool {
        self.header.sh_type == section_header::SHT_RELA
    }

    fn len(&self) -> usize {
        let sizeof_relocation = if self.is_rela { reloc::SIZEOF_RELA } else { reloc::SIZEOF_REL };

        self.header.sh_size as usize / sizeof_relocation
    }
}

impl<'a> Iterator for ElfSegments<'a> {
    type Item = &'a program_header::ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.header.e_phnum as usize {
            let item = unsafe {
                &* ((
                        self.data.as_ptr() as usize
                        + self.header.e_phoff as usize
                        + self.i * self.header.e_phentsize as usize
                    ) as *const program_header::ProgramHeader)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> Iterator for ElfSections<'a> {
    type Item = &'a section_header::SectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.header.e_shnum as usize {
            let item = unsafe {
                &* ((
                        self.data.as_ptr() as usize
                        + self.header.e_shoff as usize
                        + self.i * self.header.e_shentsize as usize
                    ) as *const section_header::SectionHeader)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Relocs<'a> {
    type Item = reloc::Reloc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len() {
            if self.is_rela {
                let item = unsafe {
                    &* ((
                            self.data.as_ptr() as usize
                            + self.header.sh_offset as usize
                            + self.i * reloc::SIZEOF_RELA
                        ) as *const reloc::Rela)
                };

                self.i += 1;
                Some(reloc::Reloc::from(item.clone()))
            } else {
                let item = unsafe {
                    &* ((
                            self.data.as_ptr() as usize
                            + self.header.sh_offset as usize
                            + self.i * reloc::SIZEOF_REL
                        ) as *const reloc::Rel)
                };

                self.i += 1;
                Some(reloc::Reloc::from(item.clone()))
            }
        } else {
            None
        }
    }
}
