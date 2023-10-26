use avr_device::atmega328p::{PORTD, USART0};
use core::{
    cell::{RefCell, RefMut},
    mem::MaybeUninit,
};

struct BaudRate {
    ubrr: u16,
    u2x: bool,
}

// Eventually u16::try_from(value).unwrap() may be usable in CTFE,
// which would replace this.
#[allow(clippy::cast_possible_truncation)]
const fn u32_as_u16(value: u32) -> u16 {
    assert!(value >= (u16::MIN as u32));
    assert!(value <= (u16::MAX as u32));
    value as u16
}

impl BaudRate {
    const fn new(baud: u32) -> Self {
        let mut ubrr = (16_000_000 / 4 / baud - 1) / 2;
        let mut u2x = true;
        if ubrr > 4095 {
            u2x = false;
            ubrr = (16_000_000 / 8 / baud - 1) / 2;
        }

        BaudRate {
            ubrr: u32_as_u16(ubrr),
            u2x,
        }
    }
}

const BAUDRATE: BaudRate = BaudRate::new(460_800);

struct Uart<const N: usize> {
    regs: MaybeUninit<USART0>,
    data: [u8; N],
    read_ix: usize,
    write_ix: usize,
}

impl<const N: usize> Uart<N> {
    const fn new() -> Self {
        assert!(N.is_power_of_two());
        // Larger don't work on AVR.
        assert!((N - 1) <= u8::MAX as usize);
        Self {
            regs: MaybeUninit::uninit(),
            data: [0u8; N],
            read_ix: 0,
            write_ix: 0,
        }
    }

    fn init(&self) {
        let regs = unsafe { self.regs.assume_init_ref() };
        // Init serial
        regs.ubrr0.write(|w| w.bits(BAUDRATE.ubrr));
        regs.ucsr0a.write(|w| w.u2x0().bit(BAUDRATE.u2x));
        // Enable receiver and transmitter
        regs.ucsr0b.write(|w| w.txen0().set_bit());
        // 8n1
        regs.ucsr0c.write(|w| {
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

    fn tx(&mut self) {
        let regs = unsafe { self.regs.assume_init_ref() };
        if regs.ucsr0a.read().udre0().bit_is_clear() {
            return;
        }
        if (self.read_ix & (N - 1)) == (self.write_ix & (N - 1)) {
            // buffer empty, disable interupt
            regs.ucsr0b
                .write(|w| w.txen0().set_bit().udrie0().clear_bit());
        } else {
            regs.udr0
                .write(|w| w.bits(self.data[self.read_ix & (N - 1)]));
            self.read_ix += 1;
        }
    }

    fn push_bytes(&mut self, data: &[u8]) -> usize {
        let regs = unsafe { self.regs.assume_init_ref() };
        let was_empty = self.read_ix == self.write_ix;
        let mut bytes_written = 0_usize;
        for c in data {
            // If the read and write indexes would meet, then the buffer is full.
            if ((self.write_ix + 1) & (N - 1)) == (self.read_ix & (N - 1)) {
                break;
            }
            self.data[self.write_ix & (N - 1)] = *c;
            self.write_ix += 1;
            bytes_written += 1;
        }
        if was_empty && bytes_written > 0 {
            // Enable interupts
            regs.ucsr0b
                .write(|w| w.txen0().set_bit().udrie0().set_bit());
            self.tx();
        }
        bytes_written
    }
}

const UART_SIZE: usize = 8;
static UART: avr_device::interrupt::Mutex<RefCell<Uart<UART_SIZE>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(Uart::new()));

fn with_uart<F, R>(f: F) -> R
where
    F: FnOnce(RefMut<Uart<UART_SIZE>>) -> R,
{
    avr_device::interrupt::free(|cs| {
        let uart = UART.borrow(cs).borrow_mut();
        f(uart)
    })
}

#[avr_device::interrupt(atmega328p)]
fn USART_UDRE() {
    with_uart(|mut uart| uart.tx());
}

/// Register a serial uart
/// SAFETY: This must be called before any logging macros.
pub fn init(portd: &PORTD, usart: USART0) {
    // Serial uses D 0 input, D 1 output
    portd.ddrd.write(|w| w.pd0().clear_bit());
    portd.ddrd.write(|w| w.pd1().set_bit());
    with_uart(|mut uart| {
        uart.regs.write(usart);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        uart.init();
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    });
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut data = s.as_bytes();
        loop {
            let bytes_written = with_uart(|mut uart| uart.push_bytes(data));
            if bytes_written == data.len() {
                break;
            } else if bytes_written > 0 {
                data = &data[bytes_written..];
            }
            avr_device::asm::sleep();
        }
        Ok(())
    }
}
