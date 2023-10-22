use avr_device::atmega328p::USART0;

pub struct Uart {
    usart: USART0,
}

impl Uart {
    pub fn new(usart: USART0) -> Self {
        let uart = Self { usart };
        uart.init();
        uart
    }

    fn init(&self) {
        // Init serial
        // baudrate 57600 16MHz clock, ubrr = (clock_freq / 4 / baud - 1) / 2;
        self.usart.ubrr0.write(|w| w.bits(34));
        self.usart.ucsr0a.write(|w| w.u2x0().bit(true));
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
