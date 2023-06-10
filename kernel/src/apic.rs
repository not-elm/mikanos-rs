use core::ffi::c_void;

use kernel_lib::acpi;
use kernel_lib::apic::device_config::LocalApicTimerDivide;
use kernel_lib::error::KernelResult;
use kernel_lib::timer::apic::local_apic_timer::LocalApicTimer;
use kernel_lib::timer::apic::ApicTimer;

pub fn start_timer(rsdp: Option<*const c_void>) -> KernelResult<()> {
    let fadt = acpi::init_acpi_timer(rsdp)?;
    let mut apic_timer = LocalApicTimer::new();

    apic_timer.start(u32::MAX, LocalApicTimerDivide::By1);
    fadt.wait_milli_for(1000);
    let elapsed = apic_timer.elapsed();
    apic_timer.stop();

    apic_timer.start(elapsed, LocalApicTimerDivide::By1);

    Ok(())
}
