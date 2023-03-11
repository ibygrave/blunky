#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut count = 0u16;

    ufmt::uwriteln!(&mut serial, "Yo Monde!").void_unwrap();
    loop {
        led.toggle();
        arduino_hal::delay_ms(count);
        count += 1;
        if count == 100 {
            ufmt::uwriteln!(&mut serial, "Toggle").void_unwrap();
            count = 0;
        }
    }
}
