use avr_device::atmega328p::PORTB;

pub struct Led {
    portb: PORTB,
}

impl Led {
    pub fn new(portb: PORTB) -> Self {
        // Configure bit 5 of port B as output:
        portb.ddrb.write(|w| w.pb5().set_bit());
        Self { portb }
    }
    pub fn on(&self) {
        self.portb.portb.write(|w| w.pb5().clear_bit());
    }
    pub fn off(&self) {
        self.portb.portb.write(|w| w.pb5().set_bit());
    }
}
