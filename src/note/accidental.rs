use std::fmt;
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Accidental(isize);

impl Accidental {
    pub const DOUBLE_FLAT: Self = Accidental::new(-2);
    pub const FLAT: Self = Accidental::new(-1);
    pub const NATURAL: Self = Accidental::new(0);
    pub const SHARP: Self = Accidental::new(1);
    pub const DOUBLE_SHARP: Self = Accidental::new(2);
    pub const fn new(pitch_shift: isize) -> Self {
        Accidental(pitch_shift)
    }
    pub const fn name(&self) -> &'static str {
        match *self {
            Accidental::DOUBLE_FLAT => "\u{1d12b}",
            Accidental::FLAT => "\u{266d}", // ♭
            Accidental::NATURAL => "\u{266e}", // ♮
            Accidental::SHARP => "\u{266f}", // ♯
            Accidental::DOUBLE_SHARP => "\u{1d12a}",
            _ => "\u{25a1}", // □
        }
    }
    pub const fn pitch_shift(self) -> isize {
        self.0
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Accidental {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\u{1d12b}" | "\u{266d}\u{266d}" | "bb" => Ok(Accidental::DOUBLE_FLAT),
            "\u{266d}" | "b" => Ok(Accidental::FLAT),
            "\u{266e}" | "" | "n" => Ok(Accidental::NATURAL),
            "\u{266f}" | "#" | "s" => Ok(Accidental::SHARP),
            "\u{1d12a}" | "x" | "\u{266f}\u{266f}" | "##" | "ss" => Ok(Accidental::DOUBLE_SHARP),
            _ => Err(()),
        }
    }
}

impl From<Accidental> for isize {
    fn from(accidental: Accidental) -> Self {
        accidental.pitch_shift()
    }
}

impl From<isize> for Accidental {
    fn from(accidental: isize) -> Self {
        Accidental::new(accidental)
    }
}

impl Shl<isize> for Accidental {
    type Output = Self;
    fn shl(self, rhs: isize) -> Self::Output {
        Accidental::new(self.pitch_shift() - rhs)
    }
}

impl Shr<isize> for Accidental {
    type Output = Self;
    fn shr(self, rhs: isize) -> Self::Output {
        Accidental::new(self.pitch_shift() + rhs)
    }
}

impl ShlAssign<isize> for Accidental {
    fn shl_assign(&mut self, rhs: isize) {
        self.0 -= rhs;
    }
}

impl ShrAssign<isize> for Accidental {
    fn shr_assign(&mut self, rhs: isize) {
        self.0 += rhs;
    }
}
