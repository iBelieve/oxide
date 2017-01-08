use core::cmp::min;
use spin::Mutex;
use multiboot2::BootInformation;
use bitmap::{Bitmap, BITS_PER_ITEM};
use super::paging::{Frame, PhysicalAddress, MAX_PAGES};

const PAGES_BITMAP_SIZE: usize = MAX_PAGES/BITS_PER_ITEM;
const KERNEL_OFFSET: PhysicalAddress = 0x0; // TODO: Update when we move to a higher-half

pub static ALLOCATOR: Mutex<BitmapFrameAllocator> = Mutex::new(BitmapFrameAllocator::new());

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct BitmapFrameAllocator {
    frame_bitmap: Bitmap<[u64; PAGES_BITMAP_SIZE]>,
    next_free_frame: usize
}

impl BitmapFrameAllocator {
    pub const fn new() -> BitmapFrameAllocator {
        BitmapFrameAllocator { frame_bitmap: Bitmap::new([u64::max_value(); PAGES_BITMAP_SIZE]),
                               next_free_frame: 0 }
    }

    pub fn mark_area_as_available(&mut self, address: PhysicalAddress, length: usize) {
        let start_frame = Frame::containing_address(address);
        let end_frame = Frame::containing_address(address + length);

        for frame_number in start_frame.number..(end_frame.number + 1) {
            self.frame_bitmap.set(frame_number, false);
        }
    }

    pub fn mark_area_in_use(&mut self, address: PhysicalAddress, length: usize) {
        let start_frame = Frame::containing_address(address);
        let end_frame = Frame::containing_address(address + length);

        for frame_number in start_frame.number..(end_frame.number + 1) {
            self.frame_bitmap.set(frame_number, true);
        }
    }
}

impl FrameAllocator for BitmapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
       if let Some(number) = self.frame_bitmap.first_unset(self.next_free_frame) {
           self.frame_bitmap.set(number, true);
           self.next_free_frame = number + 1;

           Some(Frame { number: number })
       } else {
           None
       }
   }

   fn deallocate_frame(&mut self, frame: Frame) {
       self.frame_bitmap.set(frame.number, false);
       self.next_free_frame = min(self.next_free_frame, frame.number);
   }
}

pub fn init(boot_info: &BootInformation, kernel_end: PhysicalAddress) {
    let mut allocator = ALLOCATOR.lock();

    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    for area in memory_map_tag.memory_areas() {
        allocator.mark_area_as_available(area.base_addr as usize, area.length as usize);
    }

    allocator.mark_area_in_use(0x0, kernel_end - KERNEL_OFFSET);
}
