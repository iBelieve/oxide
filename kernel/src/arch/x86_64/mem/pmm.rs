use spin::Mutex;
use bitmap::{Bitmap, BITS_PER_ITEM};
use super::paging::{Frame, PhysicalAddress, MAX_PAGES};

const PAGES_BITMAP_SIZE: usize = MAX_PAGES/BITS_PER_ITEM;
const KERNEL_OFFSET: PhysicalAddress = 0x0; // TODO: Update when we move to a higher-half

static ALLOCATOR: Mutex<BitmapFrameAllocator> = Mutex::new(BitmapFrameAllocator::new());

pub trait FrameAllocator {
    fn allocate(&mut self) -> Option<Frame>;
    fn deallocate(&mut self, frame: Frame);
    fn mark_area_in_use(&mut self, address: PhysicalAddress, length: usize);
    fn mark_frame_in_use(&mut self, frame: Frame);
}

pub struct BitmapFrameAllocator {
    frame_bitmap: Bitmap<[u64; PAGES_BITMAP_SIZE]>
}

impl BitmapFrameAllocator {
    pub const fn new() -> BitmapFrameAllocator {
        BitmapFrameAllocator { frame_bitmap: Bitmap::new([0; PAGES_BITMAP_SIZE]) }
    }
}

impl FrameAllocator for BitmapFrameAllocator {
    fn allocate(&mut self) -> Option<Frame> {
       if let Some(number) = self.frame_bitmap.first_unset(0) {
           self.frame_bitmap.set(number, true);

           Some(Frame { number: number })
       } else {
           None
       }
   }

   fn deallocate(&mut self, frame: Frame) {
       self.frame_bitmap.set(frame.number, false)
   }

   fn mark_area_in_use(&mut self, address: PhysicalAddress, length: usize) {
       let start_frame = Frame::for_address(address);
       let end_frame = Frame::for_address(address + length);

       for frame_number in start_frame.number..end_frame.number {
           self.mark_frame_in_use(Frame { number: frame_number });
       }
   }

   fn mark_frame_in_use(&mut self, frame: Frame) {
       self.frame_bitmap.set(frame.number, true);
   }
}

pub fn init(kernel_end: PhysicalAddress) {
    let mut allocator = ALLOCATOR.lock();

    allocator.mark_area_in_use(0x0, kernel_end - KERNEL_OFFSET);
}
