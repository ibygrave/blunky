#![no_std]
#![no_main]

use core::fmt::Write;
use heapless::String;
use panic_halt as _;

mod codes;
mod fizzbuzz;
mod morser;
mod serial_logger;
mod timing;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led_pin = pins.d13.into_output();
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let log_level = if cfg!(profile = "release") {
        log::Level::Error
    } else {
        log::Level::Info
    };
    serial_logger::init(serial, log_level).unwrap();
    let mut morser = morser::Morser::new(&mut led_pin);
    let mut sbuf: String<22> = String::new();

    loop {
        for fb in fizzbuzz::FizzBuzzIter::default() {
            sbuf.clear();
            write!(&mut sbuf, "{fb} ").unwrap();
            morser.emit_string(&sbuf).unwrap();
        }
    }
}
