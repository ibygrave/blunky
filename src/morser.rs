use embedded_hal::digital::v2::OutputPin;
use log::info;

use crate::codes::{MorseChar, MORSE_CODES};
use crate::timing::{T_ICS_MS, T_IWS_MS};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Morser<'l, P, E>
where
    P: OutputPin<Error = E>,
{
    led: &'l mut P,
    want_iws: bool,
}

impl<'l, P, E> Morser<'l, P, E>
where
    P: OutputPin<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(led: &'l mut P) -> Self {
        let m = Self {
            led,
            want_iws: false,
        };
        m.led.set_low().unwrap();
        info!("Starting {} {}.", PKG_NAME, PKG_VERSION);
        m
    }

    fn emit_code(&mut self, code: MorseChar) -> Result<(), E> {
        if code.is_space() {
            info!("SPACE");
            // Consecutive spaces only generate one IWS.
            self.want_iws = true;
        } else {
            arduino_hal::delay_ms(if self.want_iws { T_IWS_MS } else { T_ICS_MS });
            self.want_iws = false;
            code.emit(self.led)?;
        }
        Ok(())
    }

    pub fn emit_string(&mut self, text: &str) -> Result<(), E> {
        for c in text.chars() {
            info!("'{}'", c);
            self.emit_code(MORSE_CODES[c.to_ascii_lowercase() as usize])?;
        }
        Ok(())
    }
}
