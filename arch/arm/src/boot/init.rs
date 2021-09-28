use super::cpu;
use crate::include::asm::wfe;
use crate::kernel::{device, setup};
use crate::mm::early_memory_init;

#[entry]
fn init() -> ! {
    unsafe {
        early_memory_init();
        cpu::cpuid();
        device::device_init();
        setup::setup_kernel();
        wfe();
    }
}
