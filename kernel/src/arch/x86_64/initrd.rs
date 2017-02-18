use multiboot2::BootInformation;
use core::slice;
use tar::*;
use nom::IResult;
use filesystem::{self, TarFilesystem};
use elf::Elf;

pub fn init(boot_info: &BootInformation) {
    let initrd = boot_info.module_tags().find(|m| m.name() == "initrd")
        .expect("Missing initrd from the multiboot config");

    let bytes = unsafe {
        slice::from_raw_parts(initrd.start_address() as *const u8,
                              (initrd.end_address() - initrd.start_address()) as usize)
    };

    match parse_tar(bytes) {
        IResult::Done(_, entries) => {
            for entry in entries.iter() {
                if entry.header.name.ends_with(".ko") {
                    read(entry.contents)
                }
            }

            let fs = TarFilesystem::new(entries);

            filesystem::mount("/initrd", box fs);
        },
        e  => {
            fail!("error or incomplete: {:?}", e);
            panic!("cannot parse tar archive");
        }
    }
}

pub fn read(bytes: &[u8]) {
    match Elf::from(bytes) {
        Ok(elf) => {
            println!("Loaded elf!");
        },
        Err(e)  => {
            println!("error: {}", e);
        }
    }
}
