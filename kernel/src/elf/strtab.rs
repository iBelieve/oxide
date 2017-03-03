use core::str;
use super::Section;

pub struct Strtab<'a> {
    bytes: &'a [u8],
    delim: u8,
}

impl<'a> Strtab<'a> {
    pub fn default() -> Strtab<'static> {
        Strtab {
            bytes: &[],
            delim: 0x0,
        }
    }

    pub fn new(section: &Section<'a>, delim: u8) -> Strtab<'a> {
        Strtab {
            bytes: section.data,
            delim: delim,
        }
    }

    pub fn get(&self, idx: usize) -> &'a str {
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

