use multiboot2::BootInformation;
use core::slice;
use core::str::from_utf8;
use tar::*;
use nom::IResult;

pub fn init(boot_info: &BootInformation) {
    let initrd = boot_info.module_tags().find(|m| m.name() == "initrd")
        .expect("Missing initrd from the multiboot config");

    let bytes = unsafe {
        slice::from_raw_parts(initrd.start_address() as *const u8,
                              (initrd.end_address() - initrd.start_address()) as usize)
    };

    match parse_tar(bytes) {
        IResult::Done(_, entries) => {
            for e in entries.iter() {
                println!("{}: {}", e.header.name, from_utf8(e.contents).unwrap());
            }
        }
        e  => {
            println!("error or incomplete: {:?}", e);
            panic!("cannot parse tar archive");
        }
    }
}
