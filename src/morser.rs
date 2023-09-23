use embedded_hal::digital::v2::OutputPin;
use void::ResultVoidExt;

use crate::codes::{MorseChar, MORSE_CODES};
use crate::timing::{T_ICS_MS, T_IWS_MS};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Morser<'l, P, E, W>
where
    P: OutputPin<Error = E>,
    W: ufmt::uWrite<Error = void::Void>,
{
    led: &'l mut P,
    want_iws: bool,
    writer: &'l mut W,
}

impl<'l, P, E, W> Morser<'l, P, E, W>
where
    P: OutputPin<Error = E>,
    E: core::fmt::Debug,
    W: ufmt::uWrite<Error = void::Void>,
{
    pub fn new(led: &'l mut P, writer: &'l mut W) -> Self {
        let m = Self {
            led,
            want_iws: false,
            writer,
        };
        m.led.set_low().unwrap();
        ufmt::uwriteln!(m.writer, "Starting {} {}.\n", PKG_NAME, PKG_VERSION).void_unwrap();
        m
    }

    fn emit_code(&mut self, code: MorseChar) -> Result<(), E> {
        if code.is_space() {
            ufmt::uwrite!(self.writer, "SPACE\n").void_unwrap();
            // Consecutive spaces only generate one IWS.
            self.want_iws = true;
        } else {
            arduino_hal::delay_ms(if self.want_iws { T_IWS_MS } else { T_ICS_MS });
            self.want_iws = false;
            code.emit(self.led, self.writer)?;
        }
        Ok(())
    }

    pub fn emit_string(&mut self, text: &str) -> Result<(), E> {
        for c in text.chars() {
            ufmt::uwrite!(self.writer, "'{}': ", c).void_unwrap();
            self.emit_code(MORSE_CODES[c.to_ascii_lowercase() as usize])?;
        }
        Ok(())
    }
}
