use super::cpu::{cpu_init, id};
use super::kernel::*;
use super::modules::x86_modules_init;
use ps2::mouse::{ps2_mouse_init, MOUSE};
use ps2::test::ps2_keyboard_test;

pub unsafe fn x86_kernel_init() {
    id::get_cpuid();
    kinfo!("Got cpuid");
    x86_printk!("    CPU brand: {}", id::BRAND);

    cpu_init();
    kinfo!("CPU initialized");
    x86_printk!("    GDT initialized");
    x86_printk!("    Brand specific CPU initialized");

    x86_printk!("Running PS2 input tests...");
    if !ps2_keyboard_test() {
        x86_printk!("    PS2 keyboard test failed");
    } else {
        x86_printk!("    PS2 keyboard test passed");
    }
    kinfo!("PS2 input tests finished");

    ps2_mouse_init();
    kinfo!("PS2 mouse initialized");
    x86_printk!("    Mouse x: {}", MOUSE.lock().get_state().get_x());
    x86_printk!("    Mouse y: {}", MOUSE.lock().get_state().get_y());

    x86_modules_init();

    extern "C" { fn kernel_main(); }

    kernel_main();
}