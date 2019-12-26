use std::ops::RangeInclusive;

pub fn main(range: RangeInclusive<usize>) -> usize {
    range.filter(|i| check_password(*i)).count()
}

//However, they do remember a few key facts about the password:
//
//It is a six-digit number.
//The value is within the range given in your puzzle input.
//Two adjacent digits are the same (like 22 in 122345).
//Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn check_password(pass: usize) -> bool {
    let mut has_same_adjecent = false;
    let mut maybe_last_digit: Option<usize> = None;
    for digit in Digits::new(pass) {
        match maybe_last_digit {
            Some(last_digit) if last_digit == digit => { has_same_adjecent = true },
            Some(last_digit) if digit < last_digit => { return false },
            _ => {},
        }
        maybe_last_digit = Some(digit);
    }
    has_same_adjecent
}

// implementation taken from https://stackoverflow.com/a/41536521
struct Digits {
    n: usize,
    divisor: usize,
}
impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits { n, divisor }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}