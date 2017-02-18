use collections::Vec;
use alloc::boxed::Box;
use tar::*;
use super::{FileDescriptor, Filesystem};

fn normalized_entry_name<'a>(entry: &TarEntry<'a>) -> &'a str {
    if entry.header.name.starts_with("./") {
        &entry.header.name[2..]
    } else {
        entry.header.name
    }
}

pub struct TarFilesystem<'a> {
    entries: Vec<TarEntry<'a>>
}

impl<'a> TarFilesystem<'a> {
    pub fn new(entries: Vec<TarEntry<'a>>) -> Self {
        TarFilesystem { entries: entries }
    }
}

impl<'a> Filesystem for TarFilesystem<'a> {
    fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>> {
        for entry in self.entries.iter() {
            if normalized_entry_name(entry) == path {
                return Some(box TarFileDescriptor::new(entry));
            }
        }

        None
    }
}

pub struct TarFileDescriptor {
    contents: Vec<u8>
}

impl TarFileDescriptor {
    fn new(entry: &TarEntry) -> Self {
        TarFileDescriptor { contents: entry.contents.to_vec() }
    }
}

impl<'a> FileDescriptor for TarFileDescriptor {
    fn read(&mut self) -> Vec<u8> {
        self.contents.clone()
    }
}
