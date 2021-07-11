use crate::nrf::nrf_init;

pub static mut DEVICE: Board = Board::None;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Board {
    Nrf52840,
    Nrf52832,
    None,
}


pub(crate) unsafe fn device_init() {
    // Set device value
    #[cfg(feature = "nrf52840")]
    DEVICE = Board::Nrf52840;

    #[cfg(feature = "nrf52832")]
    DEVICE = Board::Nrf52832;

    // Initialize device
    #[cfg(feature = "nrf")]
    nrf_init();
}