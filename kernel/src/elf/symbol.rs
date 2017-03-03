use alloc::String;
use super::{Elf, Section, Strtab, Error, sym, section_header};

pub struct Symbol<'a> {
    elf: &'a Elf<'a>,
    symtab: Section<'a>,
    sym: &'a sym::Sym,
}

impl<'a> Symbol<'a> {
    pub fn new(symtab: Section<'a>, sym: &'a sym::Sym) -> Symbol<'a> {
        Symbol {
            elf: symtab.elf,
            symtab: symtab,
            sym: sym,
        }
    }

    pub fn value(&self) -> Result<usize, Error> {
        if self.st_shndx() == section_header::SHN_UNDEF as usize {
            // External symbol
            if self.st_bind() & sym::STB_WEAK != 0 {
                Ok(0)
            } else {
                let name = try!(self.st_name());
                Err(Error::UndefinedSymbol(String::from(name)))
            }
        } else if self.st_shndx() == section_header::SHN_ABS as usize {
            // Absolute symbol
            Ok(self.st_value() as usize)
        } else {
            // Internally defined symbol
            let target_section = try!(self.elf
                .section(self.st_shndx())
                .ok_or(Error::Malformed(format!("Section for internal symbol not found: {}",
                                                self.st_shndx()))));

            Ok(target_section.offset(self.st_value() as usize))
        }
    }

    fn st_name(&self) -> Result<&str, Error> {
        let strtab_section = try!(self.elf
            .section(self.symtab.sh_link())
            .ok_or(Error::Malformed(format!("Symbol strtab out of range: {}",
                                            self.symtab.sh_link()))));
        let strtab = Strtab::new(&strtab_section, 0x0);

        Ok(strtab.get(self.sym.st_name as usize))
    }
    fn st_info(&self) -> u8 {
        self.sym.st_info
    }
    fn st_other(&self) -> u8 {
        self.sym.st_other
    }
    fn st_shndx(&self) -> usize {
        self.sym.st_shndx as usize
    }
    fn st_value(&self) -> u64 {
        self.sym.st_value as u64
    }
    fn st_size(&self) -> u64 {
        self.sym.st_size as u64
    }

    /// Get the ST binding.
    ///
    /// This is the first four bits of the byte.
    #[inline]
    fn st_bind(&self) -> u8 {
        self.st_info() >> 4
    }
    /// Get the ST type.
    ///
    /// This is the last four bits of the byte.
    #[inline]
    fn st_type(&self) -> u8 {
        self.st_info() & 0xf
    }

    /// Checks whether this `Sym` has `STB_GLOBAL`/`STB_WEAK` binding and a `st_value` of 0
    pub fn is_import(&self) -> bool {
        let binding = self.sym.st_info >> 4;
        (binding == sym::STB_GLOBAL || binding == sym::STB_WEAK) && self.st_value() == 0
    }
    /// Checks whether this `Sym` has type `STT_FUNC`
    pub fn is_function(&self) -> bool {
        self.st_type() == sym::STT_FUNC
    }
}
