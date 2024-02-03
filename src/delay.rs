// Delay implementation derived from avr-hal-generic
// for 16MHz clock

use core::arch::asm;

#[allow(unused_assignments)]
fn busy_loop(mut c: u16) {
    unsafe {
        asm!(
            "1:",
            "sbiw {c}, 1",
            "brne 1b",
            c = inout(reg_iw) c,
        );
    }
}

pub fn wait_us(mut us: u16) {
    if us <= 1 {
        return;
    }
    us <<= 2;
    us -= 5;
    busy_loop(us);
}

pub fn wait_ms(ms: u16) {
    let us = u32::from(ms) * 1000;
    let iters = us >> 12;
    let mut i = 0;
    while i < iters {
        wait_us(0xfff);
        i += 1;
    }
    wait_us((us & 0xfff) as u16);
}
