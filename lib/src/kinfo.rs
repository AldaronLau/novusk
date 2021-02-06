use arch::ARCH;
use core::fmt::{Arguments, Write};
use crate::kprint::_kprint;

extern "C" {
    fn kernel_time() -> f32;
}

fn x86_kinfo(args: Arguments) {
    use arch::x86::lib::print::*;
    write!(WRITER.lock(), "[ {} ] {}", unsafe { kernel_time() }, args).unwrap();
}

pub fn _kinfo(arg: Arguments) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86_kinfo(arg)
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::kinfo::_kinfo(format_args!($($arg)*))};
}