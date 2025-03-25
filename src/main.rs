#![no_std]
#![no_main]

use core::fmt::Write;

mod codes;
mod delay;
mod fizzbuzz;
mod led;
mod morser;
mod panic;
mod timing;
mod uart;

#[macro_use]
mod macros;

#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __avr_device_rt_main_trampoline() {
    __avr_device_rt_main()
}

fn __avr_device_rt_main() -> ! {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();
    let led = led::Led::new(dp.PORTB);

    uart::init(dp.PORTD, dp.USART0);

    let mut morser = morser::Morser::new(led);

    loop {
        for fb in fizzbuzz::FizzBuzzIter::default() {
            write!(&mut morser, "{fb} ").unwrap();
        }
    }
}
