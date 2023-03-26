use core::{cmp, ptr};

use spin::mutex::{SpinMutex, SpinMutexGuard};
use uefi::table::boot::{MemoryMapIter, MemoryType};
use x86_64::structures::paging::Size2MiB;
use x86_64::{
    structures::paging::{frame::PhysFrameRange, FrameAllocator, PhysFrame},
    PhysAddr,
};

use crate::error::{KernelError, KernelResult};
use crate::paging::PAGE_SIZE_2M;

const fn kib(kib: u64) -> u64 {
    kib * 1024
}

const fn mib(mib: u64) -> u64 {
    mib * kib(1024)
}

const fn gib(gib: u64) -> u64 {
    gib * mib(1024)
}

pub(crate) const BYTES_PER_FRAME: u64 = kib(4);
const MAX_PHYSICAL_MEMORY_BYTE: u64 = gib(128);
const FRAME_COUNT: u64 = MAX_PHYSICAL_MEMORY_BYTE / BYTES_PER_FRAME;

type MapLine = u64;

const BITS_PER_MAP_LINE: u64 = MapLine::BITS as u64;
const ALLOC_MAP_LEN: usize = (FRAME_COUNT / (BITS_PER_MAP_LINE as u64)) as usize;

pub struct BitmapMemoryManager {
    alloc_map: [MapLine; ALLOC_MAP_LEN],
    range: PhysFrameRange<x86_64::structures::paging::Size2MiB>,
}

static MEMORY_MANAGER: SpinMutex<BitmapMemoryManager> = SpinMutex::new(BitmapMemoryManager {
    alloc_map: [0; ALLOC_MAP_LEN],
    range: PhysFrameRange {
        start: unsafe {
            PhysFrame::from_start_address_unchecked(PhysAddr::new_truncate(
                MAX_PHYSICAL_MEMORY_BYTE,
            ))
        },
        end: unsafe { PhysFrame::from_start_address_unchecked(PhysAddr::new_truncate(0)) },
    },
});

pub fn lock_memory_manager() -> SpinMutexGuard<'static, BitmapMemoryManager> {
    MEMORY_MANAGER.lock()
}

fn is_available(memory_type: MemoryType) -> bool {
    match memory_type {
        MemoryType::BOOT_SERVICES_CODE
        | MemoryType::BOOT_SERVICES_DATA
        | MemoryType::CONVENTIONAL => true,
        _ => false,
    }
}

impl BitmapMemoryManager {
    pub fn init(&mut self, regions: &MemoryMapIter) -> KernelResult {
        let frame_size = PAGE_SIZE_2M as u64;

        let mut available_start = self.range.start;
        let mut available_end =
            PhysFrame::from_start_address(PhysAddr::new(34359738368 + frame_size * 1024)).unwrap();
        for region in regions.clone().into_iter() {
            let usable = is_available(region.ty);
            let start = PhysAddr::new(region.phys_start);
            let end = PhysAddr::new(region.phys_start + region.page_count * 4096);
            let (start, end) = if usable {
                (start.align_up(frame_size), end.align_down(frame_size))
            } else {
                (start.align_down(frame_size), end.align_up(frame_size))
            };
            if start >= end {
                continue;
            }

            let start = PhysFrame::from_start_address(start).unwrap();
            let end = PhysFrame::from_start_address(end).unwrap();

            if available_end < start {
                self.mark_allocated(PhysFrame::<x86_64::structures::paging::Size2MiB>::range(
                    available_end,
                    start,
                ));
            }

            if usable {
                available_start = cmp::min(available_start, start);
                available_end = cmp::max(available_end, end);
            } else {
                self.mark_allocated(PhysFrame::<x86_64::structures::paging::Size2MiB>::range(
                    start, end,
                ));
            }
        }

        self.range = PhysFrame::range(available_start, available_end);
        Ok(())
    }

    pub fn mark_allocated(&mut self, range: PhysFrameRange<Size2MiB>) {
        for frame in range {
            self.set_bit(frame, true);
        }
        // update range for faster allocation
        if self.range.start == range.start {
            self.range.start = range.end;
            while self.range.start < self.range.end && self.get_bit(self.range.start) {
                self.range.start += 1;
            }
        }
    }

    // fn mark_freed(&mut self, range: PhysFrameRange) {
    //     for frame in range {
    //         self.set_bit(frame, false)
    //     }
    //     // update range if needed
    //     if self.range.start <= range.end {
    //         self.range.start = range.start;
    //     }
    // }
    pub fn allocate_with_start(
        &mut self,
        start: u64,
        num_frames: usize,
    ) -> KernelResult<PhysFrameRange<Size2MiB>> {
        let mut start_frame = unsafe {
            PhysFrame::<x86_64::structures::paging::Size2MiB>::from_start_address_unchecked(
                PhysAddr::new(start),
            )
        };

        loop {
            let end_frame = start_frame + num_frames as u64;

            if end_frame > self.range.end {
                return Err(KernelError::ExceededFrameBufferSize);
            }

            let range =
                PhysFrame::<x86_64::structures::paging::Size2MiB>::range(start_frame, end_frame);
            if let Some(allocated) = range.clone().find(|frame| self.get_bit(*frame)) {
                start_frame = allocated + 1;

                continue;
            }

            self.mark_allocated(range);
            // unsafe {
            //     range.start.start_address();
            //     let slice = core::slice::from_raw_parts_mut(
            //         range.start.start_address().as_u64() as *mut u8,
            //         (range.end.start_address().as_u64() + PAGE_SIZE_2M as u64) as usize,
            //     );
            //     slice.fill(0);
            // }
            return Ok(range);
        }
    }
    pub fn allocate(&mut self, num_frames: usize) -> KernelResult<PhysFrameRange<Size2MiB>> {
        let mut start_frame = self.range.start;

        loop {
            let end_frame = start_frame + num_frames as u64;
            if end_frame > self.range.end {
                return Err(KernelError::ExceededFrameBufferSize);
            }

            let range = PhysFrame::range(start_frame, end_frame);

            if let Some(allocated) = range.clone().find(|frame| self.get_bit(*frame)) {
                start_frame = allocated + 1;
                continue;
            }

            self.mark_allocated(range);
            // unsafe {
            //     range.start.start_address();
            //     let slice = core::slice::from_raw_parts_mut(
            //         range.start.start_address().as_u64() as *mut u8,
            //         (range.end.start_address().as_u64() + PAGE_SIZE_2M as u64) as usize,
            //     );
            //     slice.fill(0);
            // }
            return Ok(range);
        }
    }

    // pub(crate) fn free(&mut self, range: PhysFrameRange) {
    //     for frame in range {
    //         self.set_bit(frame, false)
    //     }
    // }

    pub fn get_bit(&self, frame: PhysFrame<Size2MiB>) -> bool {
        let frame_index = frame.start_address().as_u64() / BYTES_PER_FRAME;
        let line_index = (frame_index / BITS_PER_MAP_LINE) as usize;
        let bit_index = frame_index % BITS_PER_MAP_LINE;

        (self.alloc_map[line_index] & (1 << bit_index)) != 0
    }

    fn set_bit(&mut self, frame: PhysFrame<Size2MiB>, allocated: bool) {
        let frame_index = frame.start_address().as_u64() / BYTES_PER_FRAME;
        let line_index = (frame_index / BITS_PER_MAP_LINE) as usize;
        let bit_index = frame_index % BITS_PER_MAP_LINE;

        if allocated {
            self.alloc_map[line_index] |= 1 << bit_index;
        } else {
            self.alloc_map[line_index] &= !(1 << bit_index);
        }
    }
}

unsafe impl FrameAllocator<Size2MiB> for BitmapMemoryManager {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size2MiB>> {
        self.allocate(1).map(|range| range.start).ok()
    }
}
