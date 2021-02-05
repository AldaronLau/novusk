use pc_keyboard::*;
use arch::ARCH;
use drivers::keyboard::layout::default_layout;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use drivers::keyboard::x86::{handlecontrol, scancode};

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use drivers::keyboard::arm::{handlecontrol, scancode};

pub unsafe fn keyboard_init() {
    kprint!("   Setting up keyboard for {}\n", ARCH);
    let mut keyboard= Keyboard::new(default_layout(),scancode(), handlecontrol());
    keyboard.add_byte(0x20);
}
