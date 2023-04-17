use core::alloc::Layout;

use common_lib::math::frame_count_from_bytes;
use common_lib::physical_address::PhysicalAddress;

use crate::allocator::allocate_map::AllocateMap;
use crate::allocator::FRAME_SIZE;
use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::error::KernelResult;
use crate::serial_println;

pub struct BitmapMemoryFrameManager<MemoryMap>
    where
        MemoryMap: MemoryMapFrameIterable,
{
    allocate_map: AllocateMap,
    memory_map: MemoryMap,
    frame_id: usize,
}


impl<MemoryMap> BitmapMemoryFrameManager<MemoryMap>
    where
        MemoryMap: MemoryMapFrameIterable,
{
    pub fn new(memory_map: MemoryMap) -> KernelResult<BitmapMemoryFrameManager<MemoryMap>> {
        let allocate_map = AllocateMap::new();
        Ok(Self {
            allocate_map,
            memory_map,
            frame_id: 0,
        })
    }


    pub fn allocate_frame(&mut self, layout: Layout) -> Option<PhysicalAddress> {
        let frames = frame_count_from_bytes(layout.size(), FRAME_SIZE);
        self.frame_id = 0;

        loop {
            self.return_none_if_over_frames(self.frame_id, frames)?;
            if self
                .allocate_map
                .is_allocatable_multi_frames(self.frame_id, frames)
            {
                break;
            }
            self.frame_id += 1;
        }

        serial_println!("Allocate Frames {} Frame Id = {}", frames, self.frame_id);
        self.allocate_map.mark_allocate_multi_frames(self.frame_id, frames).ok()?;
        let frame = self
            .memory_map
            .frame_at(self.frame_id)?;

        self.frame_id += frames;
        Some(frame.base_phys_addr())
    }


    pub fn free_frames(&mut self, frame_id: usize, layout: Layout) -> KernelResult {
        self.allocate_map
            .free_multi_frames(frame_id, frame_count_from_bytes(layout.size(), FRAME_SIZE))?;

        Ok(())
    }


    pub(crate) fn memory_map_mut(&mut self) -> &mut MemoryMap {
        &mut self.memory_map
    }


    fn return_none_if_over_frames(&self, base_frame_id: usize, frames: usize) -> Option<()> {
        self.return_none_if_over_id(base_frame_id + frames)
    }

    fn return_none_if_over_id(&self, frame_id: usize) -> Option<()> {
        if self.memory_map.last_id().unwrap() < frame_id {
            None
        } else {
            Some(())
        }
    }
}
