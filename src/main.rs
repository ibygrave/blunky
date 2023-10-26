#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

use core::fmt::Write;

mod codes;
mod fizzbuzz;
mod led;
mod morser;
mod panic;
mod timer;
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

    timer::init(&dp.TC0);
    uart::init(&dp.PORTD, dp.USART0);
    unsafe { avr_device::interrupt::enable() };

    let mut morser = morser::Morser::new(led);

    loop {
        for fb in fizzbuzz::FizzBuzzIter::default() {
            write!(&mut morser, "{fb} ").unwrap();
        }
    }
}
