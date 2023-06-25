use core::ffi::c_void;

use kernel_lib::acpi;
use kernel_lib::apic::device_config::LocalApicTimerDivide;
use kernel_lib::error::KernelResult;
use kernel_lib::timer::apic::local_apic_timer::LocalApicTimer;
use kernel_lib::timer::apic::ApicTimer;

pub const TIMER_FREQ: u32 = 100;

pub const TIMER_100_MILLI_INTERVAL: usize = 1;


pub fn start_timer(rsdp: Option<*const c_void>, timer_freq_milli: u32) -> KernelResult<()> {
    let fadt = acpi::init_acpi_timer(rsdp)?;
    let mut apic_timer = LocalApicTimer::new();

    apic_timer.start(u32::MAX, LocalApicTimerDivide::By1);
    fadt.wait_milli_for(timer_freq_milli);
    let elapsed = apic_timer.elapsed();
    apic_timer.stop();

    let local_apic_timer_freq = elapsed;

    let initial_count = local_apic_timer_freq;

    apic_timer.start(initial_count, LocalApicTimerDivide::By1);

    Ok(())
}
