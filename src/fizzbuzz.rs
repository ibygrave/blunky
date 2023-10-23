use core::fmt::{Display, Formatter, Result};
use core::iter::Iterator;

pub enum FizzBuzz {
    Count(u16),
    Fizz,
    Buzz,
    FizzBuzz,
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FizzBuzz::Count(c) => write!(f, "{}", c),
            FizzBuzz::Fizz => write!(f, "Fizz"),
            FizzBuzz::Buzz => write!(f, "Buzz"),
            FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
        }
    }
}

#[derive(Default)]
pub struct FizzBuzzIter {
    count: u16,
}

impl Iterator for FizzBuzzIter {
    type Item = FizzBuzz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == u16::MAX {
            return None;
        }
        self.count += 1;
        Some(match self.count % 15 {
            0 => FizzBuzz::FizzBuzz,
            3 | 6 | 9 | 12 => FizzBuzz::Fizz,
            5 | 10 => FizzBuzz::Buzz,
            _ => FizzBuzz::Count(self.count),
        })
    }
}
