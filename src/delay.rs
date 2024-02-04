pub fn wait_us(mut us: u32) {
    if us <= 1 {
        return;
    }
    // 1us is 16  cycles
    us <<= 4;
    // Already used this many cycles
    us -= 21;
    avr_device::asm::delay_cycles(us);
}

pub fn wait_ms(ms: u16) {
    wait_us(u32::from(ms) * 1000);
}
