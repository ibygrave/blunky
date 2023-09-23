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

/// The morse-code for a single character
#[derive(Copy, Clone)]
struct MorseChar {
    /// The number of dot or dash signals in this encoding.
    count: u8,
    /// The least significant `count` bits represent, starting with the LSB.
    /// A 0 represents dot and 1 represents dash.
    signal: u8,
}

impl MorseChar {
    /// Construct a `MorseChar` from a string representation
    /// consisting of only '.' (dot) and '-' (dash) characters.
    const fn from_str(s: &str) -> Self {
        // Iterating over the characters of a string in a `const fn` is challenging.
        assert!(s.len() <= 8);
        let s_bytes = s.as_bytes();
        assert!(s.len() == s_bytes.len());
        let mut count: u8 = 0;
        let mut signal: u8 = 0;
        while (count as usize) < s_bytes.len() {
            signal |= (match s_bytes[count as usize] {
                b'.' => 0,
                b'-' => 1,
                _ => panic!("Not a valid morse code string"),
            } << count);
            count += 1;
        }
        Self { count, signal }
    }

    fn is_space(self) -> bool {
        // Treat all unknown characters (including ' ') as spaces.
        self.count == 0
    }

    fn emit<P, E>(self, led: &mut P) -> Result<(), E>
    where
        P: OutputPin<Error = E>,
        E: core::fmt::Debug,
    {
        let mut signal = self.signal;
        for ix in 0..self.count {
            if ix != 0 {
                arduino_hal::delay_ms(T_DIT_MS);
            }
            led.set_high()?;
            arduino_hal::delay_ms(match signal & 1 {
                0 => T_DIT_MS,
                _ => T_DAH_MS,
            });
            led.set_low()?;
            signal >>= 1;
        }
        Ok(())
    }
}

macro_rules! morse_table {
    ($($c:expr => $m:expr,)*) => {
        {
            let no_code = MorseChar {
                count: 0,
                signal: 0,
            };
            let mut table = [no_code; 128];
            $(table[$c as usize] = MorseChar::from_str($m);)*
            table
        }
    };
}

const MORSE_CODES: [MorseChar; 128] = morse_table!(
    'a' => ".-",
    'b' => "-...",
    'c' => "-.-.",
    'd' => "-..",
    'e' => ".",
    'f' => "..-.",
    'g' => "--.",
    'h' => "....",
    'i' => "..",
    'j' => ".---",
    'k' => "-.-",
    'l' => ".-..",
    'm' => "--",
    'n' => "-.",
    'o' => "---",
    'p' => ".--.",
    'q' => "--.-",
    'r' => ".-.",
    's' => "...",
    't' => "-",
    'u' => "..-",
    'v' => "...-",
    'w' => ".--",
    'x' => "-..-",
    'y' => "-.--",
    'z' => "--.-",
    '1' => ".----",
    '2' => "..---",
    '3' => "...--",
    '4' => "....-",
    '5' => ".....",
    '6' => "-....",
    '7' => "--...",
    '8' => "---..",
    '9' => "----.",
    '0' => "-----",
    '.' => ".-.-.-",
    ',' => "--..--",
    ':' => "---...",
    '?' => "..--..",
    '\'' => ".----.",
    '-' => "-....-",
    '/' => "-..-.",
    '(' => "-..-.",
    ')' => "-.--.-",
    '"' => ".-..-.",
    '+' => ".-.-.",
    '*' => "-..-",
    '@' => ".--.-.",
);

struct Morser<'l, P, E>
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
    fn new(led: &'l mut P) -> Self {
        let m = Self {
            led,
            want_iws: false,
        };
        m.led.set_low().unwrap();
        m
    }

    fn emit_code(&mut self, code: MorseChar) -> Result<(), E> {
        if code.is_space() {
            // Consecutive spaces only generate one IWS.
            self.want_iws = true;
        } else {
            arduino_hal::delay_ms(if self.want_iws { T_IWS_MS } else { T_ICS_MS });
            self.want_iws = false;
            code.emit(self.led)?;
        }
        Ok(())
    }

    fn emit_string(&mut self, text: &str) -> Result<(), E> {
        for c in text.chars() {
            self.emit_code(MORSE_CODES[c.to_ascii_lowercase() as usize])?;
        }
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
        morser.emit_string("SOS omg. Hello World! ").unwrap();
    }
}
