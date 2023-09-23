#![no_std]
#![no_main]

use panic_halt as _;

mod codes;
mod morser;
mod timing;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led_pin = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut morser = morser::Morser::new(&mut led_pin, &mut serial);

    loop {
        morser.emit_string("SOS omg. Hello World! ").unwrap();
    }
}
