#![no_std]
#![no_main]
#![feature(ascii_char)]

use core::fmt::Write;

mod codes;
mod delay;
mod fizzbuzz;
mod led;
mod morser;
mod panic;
mod strbuf;
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
    let mut sbuf: strbuf::StrBuf<9> = strbuf::StrBuf::default();

    loop {
        for fb in fizzbuzz::FizzBuzzIter::default() {
            sbuf.clear();
            write!(&mut sbuf, "{fb} ").unwrap();
            morser.emit_string(sbuf.as_str());
        }
    }
}
