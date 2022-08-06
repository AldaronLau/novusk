#![no_std]

extern crate nmallocator;
#[macro_use] extern crate tock_registers;

pub mod board;
pub use board::RaspberryPi;
pub mod common;
pub use common::*;

#[macro_use]
#[path = "../../../kernel/irq.rs"]
pub mod irq;

#[cfg(feature = "rpi2")]
pub mod rpi2;

#[cfg(feature = "rpi3")]
pub mod rpi3;

#[cfg(feature = "rpi2")]
pub use rpi2::Rpi2;

#[cfg(feature = "rpi3")]
pub use rpi3::Rpi3;

#[cfg(feature = "rpi2")]
pub use rpi2::registers::*;

#[cfg(feature = "rpi3")]
pub use rpi3::*;


#[cfg(all(feature = "rpi3", not(feature = "rpi2")))]
#[no_mangle]
// #[export_name = "device_init"]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi3::new();
    pi.init();

    if pi.error.0 {
        return (Err(pi.error.1), "RPi 3");
    } else { return (Ok(()), "RPi 3"); }
}

#[cfg(all(feature = "rpi2", not(feature = "rpi3")))]
#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi2::new();
    pi.init();

    return (Ok(()), "RPi 2");
}

fn irq_init() {

}

define_dev_irq_init!(irq_init);
