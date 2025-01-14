use core::fmt::{Arguments, Write};
use super::io::IO;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    unsafe { IO.write_fmt(fmt); }
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::arch_printk(format_args!($($arg)*))};
}
