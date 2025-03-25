use crate::info;

use core::fmt::Write;

use crate::codes::{MorseChar, MORSE_CODES};
use crate::timing::{T_ICS_MS, T_IWS_MS};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Morser {
    led: crate::led::Led,
    want_iws: bool,
}

impl Morser {
    pub fn new(led: crate::led::Led) -> Self {
        let m = Self {
            led,
            want_iws: false,
        };
        m.led.off();
        info!("Starting {PKG_NAME} {PKG_VERSION}.\n");
        m
    }

    fn emit_code(&mut self, code: MorseChar) {
        if code.is_space() {
            info!(" SPACE\n");
            // Consecutive spaces only generate one IWS.
            self.want_iws = true;
        } else {
            crate::delay::wait_ms(if self.want_iws { T_IWS_MS } else { T_ICS_MS });
            self.want_iws = false;
            code.emit(&self.led);
            info!("\n");
        }
    }
}

impl core::fmt::Write for Morser {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            info!("'{c}':");
            self.emit_code(MORSE_CODES[c.to_ascii_lowercase() as usize]);
        }
        Ok(())
    }
}
