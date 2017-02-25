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

pub struct Relocs<'a> {
    data: &'a [u8],
    sections: Filter<ElfSections<'a>, fn(&&'a section_header::SectionHeader) -> bool>,
    current_rels: Option<SectionEntries<'a, reloc::Rel>>,
    current_relas: Option<SectionEntries<'a, reloc::Rela>>,
    i: usize,
}

impl<'a> Relocs<'a> {
    pub fn new(elf: &'a Elf) -> Relocs<'a> {
        fn is_rel(s: &&section_header::SectionHeader) -> bool {
            s.sh_type == section_header::SHT_REL || s.sh_type == section_header::SHT_RELA
        }
        let mut relocs = Relocs {
            data: elf.data,
            sections: elf.sections().filter(is_rel),
            current_rels: None,
            current_relas: None,
            i: 0,
        };
        relocs.next_section();
        relocs
    }

    fn next_section(&mut self) {
        if let Some(section) = self.sections.next() {
            if section.sh_type == section_header::SHT_REL {
                self.current_rels = Some(SectionEntries::new(self.data, section));
                self.current_relas = None;
            } else if section.sh_type == section_header::SHT_RELA {
                self.current_rels = None;
                self.current_relas = Some(SectionEntries::new(self.data, section));
            } else {
                panic!("Unexpected section type: {}", section.sh_type);
            }
        } else {
            self.current_rels = None;
            self.current_relas = None;
        }
    }
}

impl<'a> Iterator for Relocs<'a> {
    type Item = reloc::Reloc;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut rels) = self.current_rels {
            if let Some(rel) = rels.next() {
                return Some(reloc::Reloc::from(rel.clone()));
            }
        } else if let Some(ref mut relas) = self.current_relas {
            if let Some(rela) = relas.next() {
                return Some(reloc::Reloc::from(rela.clone()));
            }
        } else {
            return None;
        }

        self.next_section();
        self.next()
    }
}

