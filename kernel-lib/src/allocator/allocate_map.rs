use core::ops::{Index, IndexMut};

use common_lib::nums::FlagConvertible;

use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::allocator::{FRAME_SIZE, MAX_MEMORY_SIZE};
use crate::error::AllocateReason::OverFrame;
use crate::error::{KernelError, KernelResult};

const MAP_LINE_SIZE: usize = 128;

pub const MAX_FRAME_COUNT: usize = MAX_MEMORY_SIZE / FRAME_SIZE;

const ALLOCATE_MAP_BUFF_SIZE: usize = MAX_FRAME_COUNT / MAP_LINE_SIZE;


#[derive(Debug)]
pub struct AllocateMap {
    allocate_map_buff: [u128; ALLOCATE_MAP_BUFF_SIZE],
}

impl AllocateMap {
    pub fn from_memory_map<MemoryMap>(memory_map: &MemoryMap) -> KernelResult<Self>
    where
        MemoryMap: MemoryMapFrameIterable + Clone,
    {
        let mut me = AllocateMap::new();

        for frame in memory_map.clone() {
            me.mark_allocate_frame(frame.id())?;
        }

        Ok(me)
    }


    pub fn new() -> Self {
        Self {
            allocate_map_buff: [0; ALLOCATE_MAP_BUFF_SIZE],
        }
    }
    pub fn mark_allocate_multi_frames(
        &mut self,
        base_frame_id: usize,
        frames: usize,
    ) -> KernelResult {
        for frame_id in base_frame_id..base_frame_id + frames {
            self.mark_allocate_frame(frame_id)?;
        }

        Ok(())
    }
    pub fn mark_allocate_frame(&mut self, frame_id: usize) -> KernelResult {
        self.error_if_over_id(frame_id)?;

        let line = self[line_index(frame_id)];
        self[line_index(frame_id)] = line | (1 << bit_index(frame_id));
        Ok(())
    }


    pub fn is_allocatable_multi_frames(&self, base_frame_id: usize, frames: usize) -> bool {
        (base_frame_id..base_frame_id + frames).all(|frame_id| self.is_allocatable_frame(frame_id))
    }

    pub fn is_allocatable_frame(&self, frame_id: usize) -> bool {
        (self[line_index(frame_id)] & (1 << bit_index(frame_id))).is_false()
    }


    pub fn is_not_allocatable_frame(&self, frame_id: usize) -> bool {
        !self.is_allocatable_frame(frame_id)
    }


    pub fn free_multi_frames(&mut self, frame_id: usize, frames: usize) -> KernelResult {
        self.error_if_over_id(frame_id + frames)?;

        for id in frame_id..(frame_id + frames) {
            self.free_frame(id)?;
        }

        Ok(())
    }


    pub fn free_frame(&mut self, frame_id: usize) -> KernelResult {
        self.error_if_over_id(frame_id)?;

        let line = self[line_index(frame_id)];
        self[line_index(frame_id)] = line & !(1 << bit_index(frame_id));
        Ok(())
    }


    fn error_if_over_id(&self, frame_id: usize) -> KernelResult {
        let max_frame_id = (self.allocate_map_buff.len() * MAP_LINE_SIZE) - 1;
        if max_frame_id < frame_id {
            Err(KernelError::FailedAllocate(OverFrame {
                max_frame_id,
                frame_id,
            }))
        } else {
            Ok(())
        }
    }
}

impl Index<usize> for AllocateMap {
    type Output = u128;

    fn index(&self, index: usize) -> &Self::Output {
        &self.allocate_map_buff[index]
    }
}


impl IndexMut<usize> for AllocateMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.allocate_map_buff[index]
    }
}


fn line_index(frame_id: usize) -> usize {
    frame_id / MAP_LINE_SIZE
}


fn bit_index(frame_id: usize) -> usize {
    frame_id % MAP_LINE_SIZE
}


#[cfg(test)]
mod tests {
    use crate::allocator::allocate_map::{AllocateMap, MAX_FRAME_COUNT};

    #[test]
    fn it_allocatable_frame_not_allocated() {
        let manager = AllocateMap::new();

        assert!(manager.is_allocatable_frame(1));
    }

    #[test]
    fn it_allocatable_frames_not_allocated() {
        let map = AllocateMap::new();

        assert!(map.is_allocatable_multi_frames(1, 2));
    }

    #[test]
    fn it_not_allocatable_frame() {
        let mut map = AllocateMap::new();

        assert!(map
            .mark_allocate_frame(1)
            .is_ok());
        assert!(!map.is_allocatable_frame(1));
    }


    #[test]
    fn it_free_frame() {
        let mut map = AllocateMap::new();
        assert!(map
            .mark_allocate_frame(1)
            .is_ok());
        assert!(map.free_frame(1).is_ok());
        assert!(map.is_allocatable_frame(1));
    }


    #[test]
    fn it_free_frame_only_specified() {
        let mut map = AllocateMap::new();


        assert!(map
            .mark_allocate_frame(0)
            .is_ok());

        assert!(map
            .mark_allocate_frame(1)
            .is_ok());
        assert!(map
            .mark_allocate_frame(2)
            .is_ok());

        assert!(map.free_frame(1).is_ok());


        assert!(map.is_not_allocatable_frame(0));
        assert!(map.is_allocatable_frame(1));
        assert!(map.is_not_allocatable_frame(2));
    }


    #[test]
    fn it_allocatable_2_frames() {
        let mut map = AllocateMap::new();


        assert!(map
            .mark_allocate_frame(0)
            .is_ok());
        assert!(map
            .mark_allocate_frame(1)
            .is_ok());
        assert!(map
            .mark_allocate_frame(2)
            .is_ok());
        assert!(map.free_frame(0).is_ok());
        assert!(map.free_frame(1).is_ok());

        assert!(map.is_allocatable_multi_frames(0, 2));
    }


    #[test]
    fn it_not_allocatable_3_frames() {
        let mut map = AllocateMap::new();


        assert!(map
            .mark_allocate_frame(0)
            .is_ok());
        assert!(map
            .mark_allocate_frame(1)
            .is_ok());
        assert!(map
            .mark_allocate_frame(2)
            .is_ok());

        assert!(map.free_frame(1).is_ok());


        assert!(!map.is_allocatable_multi_frames(0, 3));
    }


    #[test]
    fn it_over_frames() {
        let mut map = AllocateMap::new();
        assert!(map
            .mark_allocate_frame(MAX_FRAME_COUNT)
            .is_err());
        assert!(map
            .free_frame(MAX_FRAME_COUNT)
            .is_err());
        assert!(map
            .free_multi_frames(MAX_FRAME_COUNT - 3, 3)
            .is_err());
    }
}
