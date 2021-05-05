use super::power::shutdown;
use super::io::VirtWriter;
global_asm!(include_str!("power/shutdown.S"));
global_asm!(include_str!("start.S"));

#[no_mangle]
pub unsafe extern "C" fn virt_init() {
    let mut writer = VirtWriter;
    writer.write_string("Starting Novusk on Aarch64 Qemu Virt\n");
}