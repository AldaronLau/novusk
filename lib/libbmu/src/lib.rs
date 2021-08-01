#![no_std]

#[macro_use] extern crate printk;

pub use libc::memory as mem;

pub unsafe fn bmu_init() {
    printk!("\nStarting bare metal userspace...");
    extern "C" { fn kernel_main(); }
}
