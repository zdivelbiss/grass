use std::convert::From;
use std::fmt::{self, Display, Write};
use std::ops::{Add, Sub};

use num_bigint::BigInt;
use num_rational::BigRational;

const PRECISION: usize = 10;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Number {
    val: BigRational,
}

impl Number {
    pub const fn new(val: BigRational) -> Number {
        Number { val }
    }

    pub fn to_integer(&self) -> BigInt {
        self.val.to_integer()
    }
}

impl From<BigInt> for Number {
    fn from(b: BigInt) -> Self {
        Number {
            val: BigRational::from_integer(b),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val.to_integer())?;
        let mut frac = self.val.fract();
        if frac != BigRational::from_integer(BigInt::from(0)) {
            f.write_char('.')?;
            for _ in 0..PRECISION {
                frac *= BigRational::from_integer(BigInt::from(10));
                write!(f, "{}", frac.to_integer())?;
                frac = frac.fract();
                if frac == BigRational::from_integer(BigInt::from(0)) {
                    break;
                }
            }
            if frac != BigRational::from_integer(BigInt::from(0)) {
                write!(f, "{}", (frac * BigRational::from_integer(BigInt::from(10))).round().to_integer())?;
            }

        }
        Ok(())
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Number {
            val: self.val + other.val,
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Number {
            val: self.val - other.val,
        }
    }
}
