use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Str(String),
    Num(f32),
}

impl Add for Atom {
    type Output = Atom;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a + b),
        }
    }
}

impl Sub for Atom {
    type Output = Atom;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a - b),
        }
    }
}

impl Mul for Atom {
    type Output = Atom;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a * b),
        }
    }
}

impl Div for Atom {
    type Output = Atom;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a / b),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Str(s) => write!(f, "{}", s),
            Atom::Num(n) => write!(f, "{}", n),
        }
    }
}
