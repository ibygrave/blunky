use embedded_hal::digital::v2::OutputPin;
use void::ResultVoidExt;

use crate::timing::{T_DAH_MS, T_DIT_MS};

/// The morse-code for a single character
#[derive(Copy, Clone)]
pub struct MorseChar {
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

    pub fn is_space(self) -> bool {
        // Treat all unknown characters (including ' ') as spaces.
        self.count == 0
    }
}

struct MorseCharIter(MorseChar);

enum MorseSignal {
    Dot,
    Dash,
}

impl core::iter::Iterator for MorseCharIter {
    type Item = MorseSignal;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.count == 0 {
            return None;
        }
        let item = match self.0.signal & 1 {
            0 => MorseSignal::Dot,
            _ => MorseSignal::Dash,
        };
        self.0.count -= 1;
        self.0.signal >>= 1;
        Some(item)
    }
}

impl MorseChar {
    fn signal_iter(self) -> MorseCharIter {
        MorseCharIter(self)
    }

    pub fn emit<P, E, W>(self, led: &mut P, writer: &mut W) -> Result<(), E>
    where
        P: OutputPin<Error = E>,
        E: core::fmt::Debug,
        W: ufmt::uWrite<Error = void::Void>,
    {
        let mut space = false;
        for signal in self.signal_iter() {
            if space {
                arduino_hal::delay_ms(T_DIT_MS);
            }
            led.set_high()?;
            arduino_hal::delay_ms(match signal {
                MorseSignal::Dot => {
                    ufmt::uwrite!(writer, "DOT ").void_unwrap();
                    T_DIT_MS
                }
                MorseSignal::Dash => {
                    ufmt::uwrite!(writer, "DASH ").void_unwrap();
                    T_DAH_MS
                }
            });
            led.set_low()?;
            space = true;
        }
        ufmt::uwrite!(writer, "\n").void_unwrap();
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

pub const MORSE_CODES: [MorseChar; 128] = morse_table!(
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
