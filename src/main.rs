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

fn morse_char<P, E>(led: &mut P, c: char) -> Result<(), E>
where
    P: OutputPin<Error = E>,
{
    match c {
        'g' => {
            // - - .
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_low()?;
        }
        'm' => {
            // - -
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
        }
        'o' => {
            // - - -
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DAH_MS);
            led.set_low()?;
        }

        's' => {
            // . . .
            led.set_high()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_low()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_high()?;
            arduino_hal::delay_ms(T_DIT_MS);
            led.set_low()?;
        }
        _ => {
            arduino_hal::delay_ms(T_IWS_MS - T_ICS_MS);
        }
    }
    arduino_hal::delay_ms(T_ICS_MS);
    Ok(())
}

fn morse_string<P, E>(led: &mut P, text: &str) -> Result<(), E>
where
    P: OutputPin<Error = E>,
{
    for char in text.chars() {
        morse_char(led, char.to_ascii_lowercase())?;
    }
    Ok(())
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Yo Monde!\n").void_unwrap();
    led.set_low();
    loop {
        morse_string(&mut led, "SOS omg").unwrap();
        arduino_hal::delay_ms(3000);
    }
}
