use kernel_lib::println;

use crate::error::OperationReason::NotReflectedValue;
use crate::error::{AllocateReason, OperationReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhc::registers::capability_registers::CapabilityRegisters;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use crate::xhc::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointer;
use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhc::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhc::registers::operational_registers::OperationalRegisters;
use crate::xhc::registers::port_registers::PortRegisters;
use crate::xhc::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSet;
use crate::xhc::registers::runtime_registers::{RuntimeRegisters, RuntimeRegistersOffset};
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event_ring::EventRing;
use crate::xhc::transfer::event_ring_table::EventRingTable;
use crate::xhc::transfer::ring::RingBase;
use crate::xhc::XhcRegistersHoldable;
use crate::VolatileAccessible;

pub mod capability_registers;
pub mod doorbell_registers;
pub mod memory_mapped_addr;
pub mod operational_registers;
pub mod port_registers;
pub mod runtime_registers;

#[derive(Debug)]
pub struct Registers {
    mmio_addr: MemoryMappedAddr,
    /// Offset: 0
    capability_registers: CapabilityRegisters,
    /// Offset: CapLength Byte
    operational_registers: OperationalRegisters,
    /// Offset: RuntimeRegistersSpaceOffset
    runtime_registers: RuntimeRegisters,
}

impl XhcRegistersHoldable for Registers {
    fn reset(&mut self) -> PciResult {
        self.operational_registers.reset_host_controller();
        Ok(())
    }

    fn run(&mut self) -> PciResult {
        println!(
            "Host Controller Error = {}",
            self.operational_registers
                .usb_sts()
                .host_controller_error()
                .read_flag_volatile()
        );

        self.operational_registers.run_host_controller();
        self.port_registers()
            .filter(|port| port.is_connect())
            .for_each(|port| {
                port.reset();
                println!("Port = {:?}", port);
            });

        Ok(())
    }

    fn setup_event_ring(
        &mut self,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<(EventRingTable, EventRing)> {
        let event_ring_table_addr = allocate_event_ring_table(
            self.runtime_registers.interrupter_register_set(),
            allocator,
        )?;
        let command_ring_addr = allocate_event_ring(
            self.runtime_registers.interrupter_register_set(),
            32,
            allocator,
        )?;
        println!("Command ring address = {:x}", command_ring_addr);

        let event_ring_table = EventRingTable::new(event_ring_table_addr, command_ring_addr)?;
        let event_ring = EventRing::new(command_ring_addr, 32);

        Ok((event_ring_table, event_ring))
    }

    fn setup_command_ring(&mut self, command_ring: &CommandRing) -> PciResult {
        self.operational_registers
            .crcr()
            .register_command_ring(command_ring.ring_base_addr())
    }

    fn setup_device_context_array(&mut self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        self.setup_device_context_max_slots()?;
        let _device_context_array_addr = self.allocate_device_context_array(allocator)?;

        Ok(())
    }
}

fn allocate_event_ring_table(
    interrupter_set: &InterrupterRegisterSet,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<u64> {
    let table_ptr_address = unsafe {
        allocator
            .try_allocate_with_align(core::mem::size_of::<u128>(), 64, 64 * 1024)?
            .address()?
    };

    interrupter_set
        .event_ring_table_base_array_address()
        .update_event_ring_segment_table_addr(table_ptr_address)?;

    Ok(interrupter_set
        .event_ring_table_base_array_address()
        .read_volatile())
}

fn allocate_event_ring(
    interrupter_set: &InterrupterRegisterSet,
    ring_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<u64> {
    let event_ring_addr = unsafe {
        allocator
            .try_allocate_with_align(core::mem::size_of::<u128>() * ring_size, 64, 64 * 1024)?
            .address()?
    };

    interrupter_set
        .event_ring_dequeue_pointer()
        .update_deque_pointer(event_ring_addr)?;

    Ok(interrupter_set
        .event_ring_dequeue_pointer()
        .read_deque_pointer())
}

impl Registers {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let capability_registers = CapabilityRegisters::new(mmio_addr)?;
        let operational_registers = OperationalRegisters::new(OperationalRegistersOffset::new(
            mmio_addr,
            capability_registers.cap_length(),
        ))?;
        let runtime_registers = RuntimeRegisters::new(RuntimeRegistersOffset::new(
            mmio_addr,
            capability_registers.rts_off(),
        ));

        Ok(Self {
            mmio_addr,
            capability_registers,
            operational_registers,
            runtime_registers,
        })
    }

    pub fn trb(&self) {
        self.runtime_registers
            .interrupter_register_set()
            .debug_trb();
    }
    pub fn setup_device_context_max_slots(&self) -> PciResult {
        setup_device_context_max_slots(
            self.operational_registers.usb_command().run_stop(),
            self.capability_registers.hcs_params1().max_slots(),
            self.operational_registers.config().max_slots_en(),
        )
    }
    pub fn port_registers(&self) -> PortRegisters {
        PortRegisters::new(
            OperationalRegistersOffset::new(self.mmio_addr, self.capability_registers.cap_length()),
            self.capability_registers.hcs_params1().max_ports(),
        )
    }

    pub fn allocate_device_context_array(
        &self,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<usize> {
        unsafe {
            allocate_device_context_array(
                self.operational_registers.dcbaap(),
                self.operational_registers.config().max_slots_en(),
                allocator,
            )
        }
    }
}

/// 接続できるデバイス数を取得して、コンフィグレジスタに設定します。
fn setup_device_context_max_slots(
    run_stop: &RunStop,
    max_slots: &NumberOfDeviceSlots,
    max_slots_en: &MaxDeviceSlotsEnabled,
) -> PciResult {
    if run_stop.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::XhcRunning,
        ));
    }
    let enable_slots = max_slots.read_volatile();
    max_slots_en.write_volatile(enable_slots);

    if max_slots.read_volatile() == enable_slots {
        Ok(())
    } else {
        Err(PciError::FailedOperateToRegister(
            OperationReason::NotReflectedValue {
                expect: enable_slots as usize,
                value: max_slots.read_volatile() as usize,
            },
        ))
    }
}

unsafe fn allocate_device_context_array(
    dcbaap: &DeviceContextBaseAddressArrayPointer,
    max_slots_en: &MaxDeviceSlotsEnabled,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<usize> {
    const DEVICE_CONTEXT_SIZE: usize = 1024;

    let alloc_size = DEVICE_CONTEXT_SIZE * (max_slots_en.read_volatile() + 1) as usize;
    let device_context_array_addr = allocator
        .allocate_with_align(alloc_size, 64, 4096)
        .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
        .address()?;

    dcbaap.update_device_context_array_addr(device_context_array_addr as u64);

    let addr = dcbaap.read_device_context_array_addr();
    if addr == device_context_array_addr as u64 {
        Ok(device_context_array_addr as usize)
    } else {
        Err(PciError::FailedOperateToRegister(NotReflectedValue {
            expect: device_context_array_addr as usize,
            value: addr as usize,
        }))
    }
}
