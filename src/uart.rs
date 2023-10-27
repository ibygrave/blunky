use core::cell::RefCell;
use core::mem::MaybeUninit;

use avr_device::atmega328p::{PORTD, USART0};

pub struct Uart {
    portd: PORTD,
    usart: USART0,
}

struct BaudRate {
    ubrr: u16,
    u2x: bool,
}

impl BaudRate {
    const fn new(baud: u32) -> Self {
        let mut ubrr = (16_000_000 / 4 / baud - 1) / 2;
        let mut u2x = true;
        debug_assert!(ubrr <= u16::MAX as u32);
        if ubrr > 4095 {
            u2x = false;
            ubrr = (16_000_000 / 8 / baud - 1) / 2;
        }

        BaudRate {
            ubrr: ubrr as u16,
            u2x,
        }
    }
}

const BAUDRATE: BaudRate = BaudRate::new(460800);
static mut WRITER: MaybeUninit<RefCell<crate::uart::Uart>> = MaybeUninit::uninit();

/// Register a serial uart
/// SAFETY: This must be called before any logging macros.
pub fn init(portd: PORTD, usart: USART0) {
    let uart = Uart { portd, usart };
    uart.init();
    unsafe {
        WRITER = MaybeUninit::new(RefCell::new(uart));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl Uart {
    fn init(&self) {
        // Serial uses D 0 input, D 1 output
        self.portd.ddrd.write(|w| w.pd0().clear_bit());
        self.portd.ddrd.write(|w| w.pd1().set_bit());
        // Init serial
        self.usart.ubrr0.write(|w| w.bits(BAUDRATE.ubrr));
        self.usart.ucsr0a.write(|w| w.u2x0().bit(BAUDRATE.u2x));
        // Enable receiver and transmitter but leave interrupts disabled.
        self.usart
            .ucsr0b
            .write(|w| w.txen0().set_bit().rxen0().set_bit());
        // 8n1
        self.usart.ucsr0c.write(|w| {
            w.umsel0()
                .usart_async()
                .ucsz0()
                .chr8()
                .usbs0()
                .stop1()
                .upm0()
                .disabled()
        });
    }

    pub fn flush(&self) {
        while self.usart.ucsr0a.read().udre0().bit_is_clear() {}
    }

    pub fn write_u8(&self, data: u8) {
        self.flush();
        self.usart.udr0.write(|w| w.bits(data));
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            self.write_u8(*c);
        }
        Ok(())
    }
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut writer = unsafe { WRITER.assume_init_ref().borrow_mut() };
        writer.write_str(s)
    }
}
