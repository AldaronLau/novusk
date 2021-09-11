use libbmu::Time;
use crate::gpio::*;
use tock_registers::interfaces::Writeable;

pub struct RpiLed {
    pub gpio: RpiGpio,
}

impl RpiLed {
    pub fn new() -> Self {
        return RpiLed { gpio: RpiGpio::new() };
    }

    pub fn init(&mut self) {
        self.gpio.GPFSEL2.write(GPFSEL2::FSEL29::Output);
    }

    pub fn led_on(&mut self) {
        self.gpio.GPSET0.write(GPSET0::O29::SET);
    }

    pub fn led_off(&mut self) {
        self.gpio.GPCLR0.write(GPCLR0::O29::CLEAR);
    }

    pub fn blink(&mut self, sleep_time: usize) {
        let mut time = Time::new();

        self.led_on();
        time.sleepc(sleep_time as i64);
        self.led_off();
    }
}

fn blink(sleep: usize) {
    let mut led = RpiLed::new();
    led.init();
    led.blink(sleep);
}

define_led_blink_function!(blink);