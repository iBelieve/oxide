//! ELF executables

use alloc::String;

use core::str;

#[cfg(target_arch = "x86")]
pub use goblin::elf32::{header, program_header, section_header};

#[cfg(target_arch = "x86_64")]
pub use goblin::elf64::{header, program_header, section_header};

/// An ELF executable
pub struct Elf<'a> {
    pub data: &'a [u8],
    header: &'a header::Header
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
}

pub struct Strtab<'a> {
    bytes: &'a [u8],
    delim: u8
}

impl<'a> Strtab<'a> {
    fn default() -> Strtab<'static> {
        Strtab { bytes: &[], delim: 0x0 }
    }

    fn from_raw(bytes: &'a [u8], delim: u8) -> Strtab<'a> {
        Strtab { bytes: bytes, delim: delim }
    }

    pub fn get(&self, idx: usize) -> &str {
        let mut i = idx;
        let len = self.bytes.len();
        if i >= len {
            return "";
        }
        let mut byte = self.bytes[i];
        // TODO: this is still a hack and getting worse and worse
        if byte == self.delim {
            return "";
        }
        while byte != self.delim && i < len {
            byte = self.bytes[i];
            i += 1;
        }
        // we drop the null terminator unless we're at the end and the byte isn't a null terminator
        if i < len || self.bytes[i - 1] == self.delim {
            i -= 1;
        }
        str::from_utf8(&self.bytes[idx..i]).unwrap()
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

    fn count(self) -> usize {
        self.header.e_phnum as usize
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

    fn count(self) -> usize {
        self.header.e_shnum as usize
    }
}
