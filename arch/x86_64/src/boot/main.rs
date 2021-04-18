use super::{cmdline::cmdline_init, cpu};
use crate::drivers;
use crate::include::kernel::{die};
use crate::kernel::{init::init, kernel_init};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die();
    }

    cmdline_init();
    drivers::early_drivers_init();

    init();
    kernel_init();
    die()
}