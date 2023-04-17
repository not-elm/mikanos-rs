use core::alloc::Layout;

use common_lib::math::frame_count_from_bytes;

use crate::allocator::allocate_map::AllocateMap;
use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::allocator::memory_map::frame_range::FrameRange;
use crate::allocator::FRAME_SIZE;
use crate::error::KernelResult;

pub struct BitmapMemoryFrameManager<MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable,
{
    allocate_map: AllocateMap,
    memory_map: MemoryMap,
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
        })
    }


    pub fn allocate(&mut self, layout: Layout) -> Option<FrameRange> {
        let frames = frame_count_from_bytes(layout.size(), FRAME_SIZE);
        self.allocate_frames(frames)
    }


    pub fn allocate_frames(&mut self, frames: usize) -> Option<FrameRange> {
        let mut frame_id = 0;


        loop {
            self.return_none_if_over_frames(frame_id, frames)?;
            if self
                .allocate_map
                .is_allocatable_multi_frames(frame_id, frames)
            {
                break;
            }
            frame_id += 1;
        }


        self.allocate_map
            .mark_allocate_multi_frames(frame_id, frames)
            .ok()?;

        let base_frame = self
            .memory_map
            .frame_at(frame_id)?;

        let end_frame = self
            .memory_map
            .frame_at(frame_id + (frames - 1))?;

        Some(FrameRange::new(base_frame, end_frame))
    }

    pub fn free_frames(&mut self, frame_id: usize, layout: Layout) -> KernelResult {
        self.allocate_map
            .free_multi_frames(frame_id, frame_count_from_bytes(layout.size(), FRAME_SIZE))?;

        Ok(())
    }


    pub fn memory_map_mut(&mut self) -> &mut MemoryMap {
        &mut self.memory_map
    }


    fn return_none_if_over_frames(&self, base_frame_id: usize, frames: usize) -> Option<()> {
        self.return_none_if_over_id(base_frame_id + frames)
    }

    fn return_none_if_over_id(&self, frame_id: usize) -> Option<()> {
        if self
            .memory_map
            .last_id()
            .unwrap()
            < frame_id
        {
            None
        } else {
            Some(())
        }
    }
}
