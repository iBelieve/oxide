use bitmap::{Bitmap, BITS_PER_ITEM};
use core::cmp::min;
use multiboot2::{BootInformation, MemoryAreaIter, MemoryArea};
use super::{Frame, PhysicalAddress};
use super::paging::MAX_FRAMES;

// const FRAME_BITMAP_SIZE: usize = MAX_FRAMES/BITS_PER_ITEM;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

// pub struct BitmapFrameAllocator {
//     frame_bitmap: Bitmap<[u64; FRAME_BITMAP_SIZE]>,
//     next_free_frame: usize
// }
//
// impl BitmapFrameAllocator {
//     pub fn new(boot_info: &BootInformation, kernel_end: PhysicalAddress) -> BitmapFrameAllocator {
//         let mut allocator = BitmapFrameAllocator {
//             frame_bitmap: Bitmap::new([u64::max_value(); FRAME_BITMAP_SIZE]),
//             next_free_frame: 0
//         };
//
//         let memory_map_tag = boot_info.memory_map_tag()
//             .expect("Memory map tag required");
//
//         for area in memory_map_tag.memory_areas() {
//             println!("Available memory region: {:#x}, size: {:#x}",
//                      area.base_addr, area.length);
//
//             allocator.mark_area_as_available(area.base_addr as usize, area.length as usize);
//         }
//
//         allocator.mark_area_in_use(0x0, kernel_end);
//
//         allocator
//     }
//
//     pub fn mark_area_as_available(&mut self, address: PhysicalAddress, length: usize) {
//         let start_frame = Frame::containing_address(address);
//         let end_frame = Frame::containing_address(address + length - 1);
//
//         if self.next_free_frame > start_frame.number {
//             self.next_free_frame = start_frame.number;
//         }
//
//         for frame in Frame::range_inclusive(start_frame, end_frame) {
//             self.frame_bitmap.set(frame.number, false);
//         }
//     }
//
//     pub fn mark_area_in_use(&mut self, address: PhysicalAddress, length: usize) {
//         let start_frame = Frame::containing_address(address);
//         let end_frame = Frame::containing_address(address + length - 1);
//         if self.next_free_frame >= start_frame.number && self.next_free_frame <= end_frame.number {
//             self.next_free_frame = end_frame.number + 1;
//         }
//
//         for frame in Frame::range_inclusive(start_frame, end_frame) {
//             self.frame_bitmap.set(frame.number, true);
//         }
//     }
// }
//
// impl FrameAllocator for BitmapFrameAllocator {
//     fn allocate_frame(&mut self) -> Option<Frame> {
//         let free_frame = {
//             if self.frame_bitmap.get(self.next_free_frame) == false {
//                 Some(self.next_free_frame)
//             } else {
//                 self.frame_bitmap.first_unset(self.next_free_frame)
//             }
//         };
//
//         if let Some(number) = free_frame {
//             self.frame_bitmap.set(number, true);
//             self.next_free_frame = number + 1;
//
//             Some(Frame { number: number })
//         } else {
//             None
//         }
//    }
//
//    fn deallocate_frame(&mut self, frame: Frame) {
//        self.frame_bitmap.set(frame.number, false);
//        self.next_free_frame = min(self.next_free_frame, frame.number);
//    }
// }

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl AreaFrameAllocator {
    pub fn new(kernel_start: usize, kernel_end: usize, multiboot_start: usize,
               multiboot_end: usize, memory_areas: MemoryAreaIter) -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas.clone().filter(|area| {
            let address = area.base_addr + area.length - 1;
            Frame::containing_address(address as usize) >= self.next_free_frame
        }).min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.base_addr as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            // "Clone" the frame to return it if it's free. Frame doesn't
            // implement Clone, but we can construct an identical frame.
            let frame = Frame { number: self.next_free_frame.number };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area.base_addr + area.length - 1;
                Frame::containing_address(address as usize)
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // `frame` is used by the kernel
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1
                };
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame()
        } else {
            None // no free frames left
        }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        // TODO: Implement!
    }
}
