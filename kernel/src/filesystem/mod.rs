use alloc::{BTreeMap, String, Vec};
use alloc::boxed::Box;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::tarfs::TarFilesystem;

mod tarfs;

static FILESYSTEM: Once<RwLock<VirtualFilesystem>> = Once::new();

pub trait Filesystem: Send + Sync {
    fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>>;
}

pub trait FileDescriptor {
    fn read(&mut self) -> Vec<u8>;
}

pub struct VirtualFilesystem {
    mounts: BTreeMap<String, Box<Filesystem>>
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        VirtualFilesystem {
            mounts: BTreeMap::new()
        }
    }

    pub fn mount(&mut self, path: &str, fs: Box<Filesystem>) {
        let mut normalized_path = String::from(path);
        if !path.ends_with("/") {
            normalized_path += "/";
        }

        self.mounts.insert(normalized_path, fs);
    }

    pub fn find_mount<'a>(&self, path: &'a str) -> Option<(&str, &'a str)> {
        // Keys are sorted, so we iterate in reverse so we get the longest paths first
        // Thus we pick /path/to/mount1/mount2 over /path/to/mount1
        for key in self.mounts.keys().rev() {
            if path.starts_with(key) {
                return Some((key, &path[key.len()..]));
            }
        }

        None
    }

    pub fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>> {
        assert!(path.starts_with("/"));

        if let Some((mount_point, sub_path)) = self.find_mount(path) {
            let fs = self.mounts.get(mount_point).unwrap();

            assert!(!sub_path.starts_with("/"));

            fs.get_file(&sub_path)
        } else {
            None
        }
    }
}

fn init_fs() -> RwLock<VirtualFilesystem> {
    RwLock::new(VirtualFilesystem::new())
}

pub fn fs() -> RwLockReadGuard<'static, VirtualFilesystem> {
    FILESYSTEM.call_once(init_fs).read()
}

fn mut_fs() -> RwLockWriteGuard<'static, VirtualFilesystem> {
    FILESYSTEM.call_once(init_fs).write()
}

pub fn mount(path: &str, fs: Box<Filesystem>) {
    mut_fs().mount(path, fs)
}
