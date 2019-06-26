
use std::ops::{Add, Sub};

#[derive(Debug)]

pub struct Decimal(i64);

impl Add for Decimal {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
        }
}

impl Sub for Decimal {
        type Output = Self;
        
        fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
        }
}

impl Decimal {
        pub fn new(m: i64, f: i64) -> Self {
                Self(m * 10i64.pow(Decimal::scale()) + f)
        }

        pub fn scale() -> u32 {
                4
        }
}