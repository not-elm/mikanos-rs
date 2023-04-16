use core::alloc::{GlobalAlloc, Layout};
use core::cell::OnceCell;
use core::marker::PhantomData;

use spin::Mutex;
use uefi::table::boot::MemoryMapIter;

use common_lib::math::frame_size_from_bytes;

use crate::allocator::allocate_map::AllocateMap;
use crate::allocator::memory_map_frame_iterable::{MemoryMapFrame, MemoryMapFrameIterable};
use crate::allocator::FRAME_SIZE;
use crate::error::{AllocateReason, KernelError, KernelResult};
use crate::serial_println;

pub struct BitmapMemoryManager<'memory, MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable<'memory> + Clone,
{
    allocate_map: AllocateMap,
    memory_map: MemoryMap,
    frame_id: usize,
    _maker: PhantomData<MemoryMapIter<'memory>>,
}


impl<'memory, MemoryMap> BitmapMemoryManager<'memory, MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable<'memory> + Clone,
{
    pub fn new(memory_map: MemoryMap) -> KernelResult<BitmapMemoryManager<'memory, MemoryMap>> {
        let allocate_map = AllocateMap::from_memory_map(&memory_map)?;
        Ok(Self {
            allocate_map,
            memory_map,
            frame_id: 0,
            _maker: PhantomData,
        })
    }


    pub fn allocate_frame(&mut self, layout: Layout) -> Option<MemoryMapFrame<'memory>> {
        let frames = frame_size_from_bytes(layout.size(), FRAME_SIZE);

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

        let frame = self
            .memory_map
            .get(self.frame_id)?;
        self.frame_id += 1;
        Some(frame)
    }


    pub fn free_frames(&mut self, frame_id: usize, layout: Layout) -> KernelResult {
        self.allocate_map
            .free_multi_frames(frame_id, layout.size())
    }

    fn return_none_if_over_frames(&self, base_frame_id: usize, frames: usize) -> Option<()> {
        self.return_none_if_over_id(base_frame_id + frames)
    }

    fn return_none_if_over_id(&self, frame_id: usize) -> Option<()> {
        return if self.memory_map.last_id() < frame_id {
            None
        } else {
            Some(())
        };
    }
}


pub struct BitmapAllocator<MemoryMap>(OnceCell<Mutex<BitmapMemoryManager<'static, MemoryMap>>>)
where
    MemoryMap: MemoryMapFrameIterable<'static> + Clone;


impl<MemoryMap> BitmapAllocator<MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable<'static> + Clone,
{
    pub const fn uninit() -> BitmapAllocator<MemoryMap> {
        Self(OnceCell::new())
    }


    pub fn init(&mut self, memory_map: MemoryMap) -> KernelResult {
        let a = Mutex::new(BitmapMemoryManager::new(memory_map)?);

        self.0
            .set(a)
            .map_err(|_| KernelError::FailedAllocate(AllocateReason::InitializeGlobalAllocator))
    }
}


unsafe impl<'memory, MemoryMap> GlobalAlloc for BitmapAllocator<MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable<'static> + Clone,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let frame = self
            .0
            .get()
            .unwrap()
            .lock()
            .allocate_frame(layout)
            .unwrap();
        frame.descriptor().phys_start as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let phs_addr = ptr as u64;
        let frame = self
            .0
            .get()
            .unwrap()
            .lock()
            .memory_map
            .find(|frame| {
                let frame_addr = frame.descriptor().phys_start;
                frame_addr <= phs_addr && phs_addr <= (frame_addr + FRAME_SIZE as u64)
            })
            .unwrap();


        self.0
            .get()
            .unwrap()
            .lock()
            .free_frames(frame.id(), layout)
            .unwrap();
    }
}
