use core::marker::PhantomData;
use core::mem;
use core::iter::Filter;
use super::{Elf, ElfSections, section_header, reloc};

pub struct SectionEntries<'a, T> {
    data: &'a [u8],
    section: &'a section_header::SectionHeader,
    i: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> SectionEntries<'a, T> {
    pub fn new(data: &'a [u8],
               section: &'a section_header::SectionHeader)
               -> SectionEntries<'a, T> {
        SectionEntries {
            data: data,
            section: section,
            i: 0,
            phantom: PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.section.sh_size as usize / mem::size_of::<T>()
    }
}

impl<'a, T> Iterator for SectionEntries<'a, T>
    where T: 'a
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.len() as usize {
            let item = unsafe {
                &*((self.data.as_ptr() as usize + self.section.sh_offset as usize +
                    self.i * mem::size_of::<T>()) as *const T)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

