#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

const WPM: u16 = 10;
const T_DIT_MS: u16 = (60000) / (50 * WPM);
const T_DAH_MS: u16 = 3 * T_DIT_MS;
const T_ICS_MS: u16 = 3 * T_DIT_MS;
const T_IWS_MS: u16 = 7 * T_DIT_MS;

struct Morser<'l, P, E>
where
    P: OutputPin<Error = E>,
{
    led: &'l mut P,
}

impl<'l, P, E> Morser<'l, P, E>
where
    P: OutputPin<Error = E>,
    E: core::fmt::Debug,
{
    fn new(led: &'l mut P) -> Self {
        let m = Self { led };
        m.led.set_low().unwrap();
        m
    }

    fn emit_char(&mut self, c: char) -> Result<(), E> {
        match c {
            'g' => {
                // - - .
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_low()?;
            }
            'm' => {
                // - -
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
            }
            'o' => {
                // - - -
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DAH_MS);
                self.led.set_low()?;
            }

            's' => {
                // . . .
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_low()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_high()?;
                arduino_hal::delay_ms(T_DIT_MS);
                self.led.set_low()?;
            }
            _ => {
                arduino_hal::delay_ms(T_IWS_MS - T_ICS_MS);
            }
        }
        arduino_hal::delay_ms(T_ICS_MS);
        Ok(())
    }

    fn emit_string(&mut self, text: &str) -> Result<(), E> {
        for char in text.chars() {
            self.emit_char(char.to_ascii_lowercase())?;
        }
        arduino_hal::delay_ms(T_IWS_MS);
        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led_pin = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut morser = Morser::new(&mut led_pin);

    ufmt::uwriteln!(&mut serial, "Yo Monde!\n").void_unwrap();
    loop {
        morser.emit_string("SOS omg").unwrap();
    }
}
