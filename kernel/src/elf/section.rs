use core::marker::PhantomData;
use core::mem;
use core::iter::Filter;
use super::{Elf, ElfSections, section_header, reloc};

pub struct Section<'a> {
    pub elf: &'a Elf<'a>,
    pub header: &'a section_header::SectionHeader,
    pub data: &'a [u8],
}

impl<'a> Section<'a> {
    pub fn new(elf: &'a Elf<'a>, header: &'a section_header::SectionHeader) -> Section<'a> {
        let start = header.sh_offset as usize;
        let end = start + header.sh_size as usize;

        Section {
            elf: elf,
            header: header,
            data: &elf.data[start..end],
        }
    }

    pub fn entries<'b, T>(&'b self) -> SectionEntries<'b, 'a, T>
        where T: 'b
    {
        SectionEntries::new(self)
    }

    pub fn entries_count<T>(&self) -> usize {
        self.data.len() as usize / mem::size_of::<T>()
    }

    pub fn entry<T>(&self, index: usize) -> Option<&'a T>
        where T: 'a
    {
        if index < self.entries_count::<T>() as usize {
            let item = unsafe {
                &*((self.data.as_ptr() as usize + index * mem::size_of::<T>()) as *const T)
            };
            Some(item)
        } else {
            None
        }
    }

    pub fn offset(&self, offset: usize) -> usize {
        self.data.as_ptr() as usize + offset
    }

    pub fn sh_name(&self) -> &str {
        self.elf.shdr_strtab().get(self.header.sh_name as usize)
    }

    /// Section type
    pub fn sh_type(&self) -> u32 {
        self.header.sh_type
    }

    /// Section flags
    pub fn sh_flags(&self) -> u32 {
        self.header.sh_flags as u32
    }

    /// Section virtual addr at execution
    pub fn sh_addr(&self) -> u64 {
        self.header.sh_addr as u64
    }

    /// Section file offset
    pub fn sh_offset(&self) -> u64 {
        self.header.sh_offset as u64
    }

    /// Section file size
    pub fn sh_size(&self) -> u64 {
        self.header.sh_size as u64
    }

    /// Link to another section
    pub fn sh_link(&self) -> usize {
        self.header.sh_link as usize
    }
}

pub struct SectionEntries<'a, 'b: 'a, T>
    where T: 'a
{
    section: &'a Section<'b>,
    i: usize,
    phantom: PhantomData<T>,
}

impl<'a, 'b, T> SectionEntries<'a, 'b, T> {
    pub fn new(section: &'a Section<'b>) -> SectionEntries<'a, 'b, T> {
        SectionEntries {
            section: section,
            i: 0,
            phantom: PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.section.entries_count::<T>()
    }
}

impl<'a, 'b, T> Iterator for SectionEntries<'a, 'b, T>
    where T: 'b
{
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.section.entry(self.i) {
            self.i += 1;
            Some(entry)
        } else {
            None
        }
    }
}

