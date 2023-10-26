use core::cell;

const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u8 = 219;
const MILLIS_INCREMENT: u32 = PRESCALER * (TIMER_COUNTS as u32) / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    });
}

pub fn init(tc0: &avr_device::atmega328p::TC0) {
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());
}

pub fn delay_ms(delay: u16) {
    let delay = u32::from(delay);
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
    while avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get()) < delay {
        avr_device::asm::sleep();
    }
}
